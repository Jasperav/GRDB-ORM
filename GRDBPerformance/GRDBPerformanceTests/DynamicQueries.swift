import GRDBPerformance
import XCTest
import GRDB

// Not really a performance test, but validates the output of the generated queries
class DynamicQueryTest: XCTestCase {
    func testAll() {
        let db = setupPool()
        
        assertAmountOfUsersBeforeInsertion(db: db)
        
        let user = DbUser(
                userUuid: UUID(),
                firstName: "name",
                jsonStruct: JsonType(age: 1),
                jsonStructOptional: nil,
                jsonStructArray: [],
                jsonStructArrayOptional: [
                    JsonType(age: 3)
                ],
                integer: 2)
        let book0 = DbBook(bookUuid: UUID(), userUuid: user.userUuid, integerOptional: 0, tsCreated: 0)
        let book1 = DbBook(bookUuid: UUID(), userUuid: user.userUuid, integerOptional: nil, tsCreated: 0)

        try! db.write { db in
            try! user.genInsert(db: db)
            try! book0.genInsert(db: db)
            try! book1.genInsert(db: db)
        }
        
        assertBooksForUserWithSpecificUuid(db: db, user: user, book0: book0, book1: book1)
        assertFindByUsername(db: db, find: user)
        assertAmountOfUsersAfterInsertion(db: db)
        assertAmountOfUsersAfterInsertion(db: db)
        assertDeleteByUserUuid(db: db, user: user)
    }

    func assertFindByUsername(db: DatabasePool, find: DbUser) {
        XCTAssert(try! DbUser.quickReadFindByUsername(db: db, firstName: "doesnotexists") == nil)
        XCTAssertEqual(find.userUuid, try! DbUser.quickReadFindByUsername(db: db, firstName: find.firstName!)!.userUuid)
    }
    
    func assertFindUserUuidByUsername(db: DatabasePool, find: DbUser) {
        XCTAssert(try! DbUser.quickReadFindUserUuidByUsername(db: db, firstName: "doesnotexists") == nil)
        XCTAssertEqual(find.userUuid, try! DbUser.quickReadFindUserUuidByUsername(db: db, firstName: find.firstName!)!)
    }

    func assertAmountOfUsersAfterInsertion(db: DatabasePool) {
        XCTAssertEqual(1, try! DbUser.quickReadAmountOfUsers(db: db))
    }
    
    func assertAmountOfUsersBeforeInsertion(db: DatabasePool) {
        XCTAssertEqual(0, try! DbUser.quickReadAmountOfUsers(db: db))
    }
    
    func assertBooksForUserWithSpecificUuid(db: DatabasePool, user: DbUser, book0: DbBook, book1: DbBook) {
        let result = try! DbBook.quickReadBooksForUserWithSpecificUuid(db: db, userUuid: user.userUuid)

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
    
    func assertDeleteByUserUuid(db: DatabasePool, user: DbUser) {
        try! db.write { db in
            try! DbBook.deleteByUserUuid(db: db, userUuid: user.userUuid)
            
            XCTAssertEqual(2, db.changesCount)
        }
    }
}
