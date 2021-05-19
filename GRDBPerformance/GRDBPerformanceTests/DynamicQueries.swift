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

            try! db.write { db in
                try! user.genInsert(db: con)
                try! book0.genInsert(db: con)
                try! book1.genInsert(db: con)
            }

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

        XCTAssertEqual(book0.integerOptional, firstRow.0.integerOptional!)
        XCTAssertEqual(user.integer, firstRow.1)
        XCTAssertEqual(user.jsonStructArrayOptional, firstRow.2)
        XCTAssertEqual(1, firstRow.3)

        // Assertion checks second row
        let secondRow = result[1]

        XCTAssertEqual(book1.integerOptional, secondRow.0.integerOptional)
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

            let check: (SerializedInfo, SerializedInfo?) -> () = {
                XCTAssertEqual($0, dbUser.serializedInfoAutoConvert())
                XCTAssertEqual($1, dbUser.serializedInfoNullableAutoConvert())
            }

            let (serialize, serializeNullable) = try! DbUser.serializeInfoSingle(db: con)!

            check(serialize, serializeNullable)

            dbUser.serializedInfoNullableAutoSet(serializedInfoNullable: .data("Something"))

            try! dbUser.primaryKey().genUpdateSerializedInfoNullable(db: con, serializedInfoNullable: dbUser.serializedInfoNullableAutoConvert())

            let (serializeUpdated, serializeNullableUpdated) = try! DbUser.serializeInfoSingle(db: con)!

            check(serializeUpdated, serializeNullableUpdated)

            let array = try! DbUser.serializeInfoArray(db: con)

            XCTAssertEqual(1, array.count)

            for (s, n) in array {
                XCTAssertEqual(s, serializeUpdated)
                XCTAssertEqual(n, serializeNullableUpdated)
            }
        }
    }
}
