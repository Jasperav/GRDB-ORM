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

        try! db.write { con in
            try! user.genInsert(db: con)

            let book = DbBook(bookUuid: UUID(), userUuid: user.userUuid, integerOptional: nil, tsCreated: 0)

            try! book.genInsert(db: con)

            let userBook = DbUserBook(bookUuid: book.bookUuid, userUuid: user.userUuid)

            try! userBook.genInsert(db: con)

            let user2 = DbUser.random()

            try! user2.genInsert(db: con)
            try! userBook.primaryKey().genUpdateUserUuid(db: con, userUuid: user2.userUuid)
            try! DbUserBook.PrimaryKey(bookUuid: book.bookUuid, userUuid: user2.userUuid).genSelectExpect(db: con)
        }
    }
}
