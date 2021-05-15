import XCTest
import GRDBPerformance
import GRDB
import Foundation

class UpdatePerformanceTest: XCTestCase {
    func testGenerated() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! DbUser(userUuid: uuid,
                    firstName: nil,
                    jsonStruct: .init(age: 1),
                    jsonStructOptional: nil,
                    jsonStructArray: [],
                    jsonStructArrayOptional: nil,
                    integer: 1,
                    bool: true,
                    serializedInfo: .data("r"),
                    serializedInfoNullable: nil)
                    .genUpdate(db: db)
        })
    }

    func testGRDB() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! User(userUuid: uuid,
                    firstName: nil,
                    jsonStruct: .init(age: 1),
                    jsonStructOptional: nil,
                    jsonStructArray: [],
                    jsonStructArrayOptional: nil,
                    integer: 1,
                    bool: true,
                    serializedInfo: Data(),
                    serializedInfoNullable: nil)
                    .update(db)
        })
    }
}

class UpdatePrimaryKeyTest: XCTestCase {
    func testUpdatePk() throws {
        let db = setupPool()
        let user = DbUser.random()

        try! user.genInsert(dbWriter: db)

        let book = DbBook(bookUuid: UUID(), userUuid: user.userUuid, integerOptional: nil, tsCreated: 0)

        try! book.genInsert(dbWriter: db)

        let userBook = DbUserBook(bookUuid: book.bookUuid, userUuid: user.userUuid)

        try! userBook.genInsert(dbWriter: db)

        let user2 = DbUser.random()

        try! user2.genInsert(dbWriter: db)
        try! userBook.primaryKey().genUpdateUserUuid(dbWriter: db, userUuid: user2.userUuid)
        try! DbUserBook.PrimaryKey(bookUuid: book.bookUuid, userUuid: user2.userUuid).genSelectExpect(dbReader: db)
    }
}
