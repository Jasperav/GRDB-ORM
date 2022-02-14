import GRDBPerformance
import XCTest
import GRDB

// Not really a performance test, but validates the output of the generated queries
class DynamicQueryTest: XCTestCase {
    func testAll() {
        let db = setupPool()

        try! db.write { con in
            assertAmountOfUsersBeforeInsertion(con: con)

            let user = DbUser.random()
            let book0 = DbBook(bookUuid: UUID(), userUuid: user.userUuid, integerOptional: 0, tsCreated: 0)
            let book1 = DbBook(bookUuid: UUID(), userUuid: user.userUuid, integerOptional: nil, tsCreated: 0)

            try! user.genInsert(db: con)
            try! book0.genInsert(db: con)
            try! book1.genInsert(db: con)

            assertBooksForUserWithSpecificUuid(con: con, user: user, book0: book0, book1: book1)
            assertFindByUsername(con: con, find: user)
            assertAmountOfUsersAfterInsertion(con: con)
            assertAmountOfUsersAfterInsertion(con: con)
            assertDeleteByUserUuid(con: con, user: user)
        }
    }

    func assertFindByUsername(con: Database, find: DbUser) {
        XCTAssert(try! DbUser.findByUsername(db: con, firstName: "doesnotexists") == nil)
        XCTAssertEqual(find.userUuid, try! DbUser.findByUsername(db: con, firstName: find.firstName!)!.userUuid)
    }

    func assertFindUserUuidByUsername(con: Database, find: DbUser) {
        XCTAssert(try! DbUser.findUserUuidByUsername(db: con, firstName: "doesnotexists") == nil)
        XCTAssertEqual(find.userUuid, try! DbUser.findUserUuidByUsername(db: con, firstName: find.firstName!)!)
    }

    func assertAmountOfUsersAfterInsertion(con: Database) {
        XCTAssertEqual(1, try! DbUser.amountOfUsers(db: con))
    }

    func assertAmountOfUsersBeforeInsertion(con: Database) {
        XCTAssertEqual(0, try! DbUser.amountOfUsers(db: con))
    }

    func assertBooksForUserWithSpecificUuid(con: Database, user: DbUser, book0: DbBook, book1: DbBook) {
        let result = try! DbBook.booksForUserWithSpecificUuid(db: con, userUuid: user.userUuid)

        XCTAssertEqual(2, result.count)

        // Assertion checks first row
        let firstRow = result[0]

        XCTAssertEqual(book0.integerOptional, firstRow.gen0.integerOptional!)
        XCTAssertEqual(user.integer, firstRow.gen1)
        XCTAssertEqual(user.jsonStructArrayOptional, firstRow.gen2)
        XCTAssertEqual(1, firstRow.gen3)

        // Assertion checks second row
        let secondRow = result[1]

        XCTAssertEqual(book1.integerOptional, secondRow.gen0.integerOptional)
        // No need to check more I guess
    }

    func assertDeleteByUserUuid(con: Database, user: DbUser) {
        try! DbBook.deleteByUserUuid(db: con, userUuid: user.userUuid)

        XCTAssertEqual(2, con.changesCount)
    }

    func testBoolReturnType() {
        let db = setupPool()

        try! db.write { con in
            XCTAssertEqual(false, try! DbBook.hasAtLeastOneBook(db: con))

            try! DbBook(bookUuid: UUID(), userUuid: nil, integerOptional: 0, tsCreated: 0).genInsert(db: con)

            XCTAssertEqual(true, try! DbBook.hasAtLeastOneBook(db: con))

            try! DbBook.genDeleteAll(db: con)

            XCTAssertEqual(false, try! DbBook.hasAtLeastOneBook(db: con))
        }
    }

    func testMappedBlobColumn() {
        let db = setupPool()

        try! db.write { con in
            XCTAssertNil(try! DbUser.serializeInfoSingle(db: con))
            XCTAssert(try! DbUser.serializeInfoArray(db: con).isEmpty)

            var dbUser = DbUser.random()

            dbUser.serializedInfoNullableAutoSet(serializedInfoNullable: nil)

            try! dbUser.genInsert(db: con)

            let check: (DbUser.SerializeInfoSingleType) -> () = {
                XCTAssertEqual($0.gen0, dbUser.serializedInfoAutoConvert())
                XCTAssertEqual($0.gen1, dbUser.serializedInfoNullableAutoConvert())
            }

            let serializeInfo = try! DbUser.serializeInfoSingle(db: con)!

            check(serializeInfo)

            dbUser.serializedInfoNullableAutoSet(serializedInfoNullable: .data("Something"))

            try! dbUser.primaryKey().genUpdateSerializedInfoNullable(db: con, serializedInfoNullable: dbUser.serializedInfoNullableAutoConvert())

            let updated = try! DbUser.serializeInfoSingle(db: con)!

            check(updated)

            let array = try! DbUser.serializeInfoArray(db: con)

            XCTAssertEqual(1, array.count)

            for u in array {
                XCTAssertEqual(u.gen0, updated.gen0)
                XCTAssertEqual(u.gen1, updated.gen1)
            }
        }
    }

    func testOptionals() throws {
        let db = setupPool()

        try db.write { con in
            let book = DbBook(bookUuid: UUID(), userUuid: nil, integerOptional: nil, tsCreated: 0)

            try book.genInsert(db: con)

            let books = try DbBook.booksWithOptionalUser(db: con)

            XCTAssertEqual(1, books.count)
            XCTAssertEqual(book, books[0].gen0)
            XCTAssertNil(books[0].gen1)
            XCTAssertNil(books[0].gen2)
        }
    }

    func testValueObservation() throws {
        let db = setupPool()
        let toSearchFor = "first"
        let publisher = DbUser.FindByUsernameQueryable(firstName: toSearchFor).publisher(in: db)
        var count = 0
        let exp = expectation(description: "count")

        let cancellable = publisher
                .sink(receiveCompletion: { _ in
                    XCTFail("Should not complete")
                }, receiveValue: { _ in
                    count += 1

                    if count == 2 {
                        exp.fulfill()
                    }
                })

        try db.write { con in
            var user = DbUser.random()

            user.firstName = toSearchFor

            try user.genInsert(db: con)

            user.userUuid = UUID()
            user.firstName = "somethingdifferent"

            try user.genInsert(db: con)

            user.userUuid = UUID()
            user.firstName = toSearchFor
        }

        waitForExpectations(timeout: 3)
    }

    func testSimpleInQuery() {
        let db = setupPool()

        try! db.write { con in
            let checkCount: (Int, [String]) -> () = {
                XCTAssertEqual($0, try! DbUser.allWithProvidedFirstNames(db: con, firstName: $1).count)
            }

            checkCount(0, [])
            checkCount(0, ["something"])

            var user0 = DbUser.random()

            user0.firstName = "something"

            try! user0.genInsert(db: con)

            checkCount(0, ["somethingelse"])
            checkCount(1, [user0.firstName!])

            var user1 = DbUser.random()

            user1.firstName = "somethingelse"

            try! user1.genInsert(db: con)

            checkCount(1, [user0.firstName!])
            checkCount(2, [user0.firstName!, user1.firstName!])
        }
    }

    func testComplexInQuery() {
        let db = setupPool()
        
        var user0 = DbUser.random()

        user0.serializedInfoNullableAutoSet(serializedInfoNullable: .data("something"))
        user0.jsonStructOptional = .init(age: 1)

        try! db.write { con in
            try! user0.genInsert(db: con)
        }
        
        try! db.write { con in
            let checkCount: (Int, [String], JsonType, [Int], SerializedInfo) -> () = { count, firstNames, jsonStructOptional, integer, serializedInfoNullable in
                XCTAssertEqual(count, try! DbUser.complex(db: con, firstNames0: firstNames, jsonStructOptional: jsonStructOptional, integer: integer, serializedInfoNullable: serializedInfoNullable).count)
            }

            checkCount(0, [], .init(age: 0), [], SerializedInfo.data("data"))
            
            checkCount(0, [user0.firstName!], user0.jsonStructOptional!, [user0.integer], .data("somethingelse"))
            checkCount(0, [], user0.jsonStructOptional!, [user0.integer], .data("somethingelse"))
            checkCount(0, [], user0.jsonStructOptional!, [], .data("somethingelse"))
            checkCount(0, [user0.firstName!], user0.jsonStructOptional!, [], .data("somethingelse"))
            checkCount(1, [user0.firstName!], user0.jsonStructOptional!, [user0.integer], user0.serializedInfoNullableAutoConvert()!)
            checkCount(1, [user0.firstName!, user0.firstName!], user0.jsonStructOptional!, [user0.integer, user0.integer], user0.serializedInfoNullableAutoConvert()!)
        }
    }
}
