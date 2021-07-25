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

class UpdateTest: XCTestCase {
    func test() throws {
        let db = setupPool()
        var user = DbUser.random()

        try! db.write { con in
            try! user.genInsert(db: con)

            let assertUser: () -> () = {
                XCTAssertEqual(user, try! user.primaryKey().genSelectExpect(db: con))
            }

            // First check with a nullable data type
            let changeValue: (SerializedInfo?) -> () = {
                user.serializedInfoNullableAutoSet(serializedInfoNullable: $0)

                try! user.primaryKey().genUpdateSerializedInfoNullable(db: con, serializedInfoNullable: user.serializedInfoNullableAutoConvert())

                assertUser()
            }

            // Make sure there is a value
            changeValue(.data("something"))

            // Make sure there is no value
            changeValue(nil)

            // Check with a nonnull type
            user.bool = !user.bool

            try! user.primaryKey().genUpdateBool(db: con, bool: user.bool)

            assertUser()
        }
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

            var userBook = DbUserBook(bookUuid: book.bookUuid, userUuid: user.userUuid, realToDouble: 123.456)

            try! userBook.genInsert(db: con)

            let user2 = DbUser.random()

            try! user2.genInsert(db: con)

            try! userBook.primaryKey().genUpdateUserUuid(db: con, userUuid: user2.userUuid)

            userBook.userUuid = user2.userUuid

            let new = try! DbUserBook.PrimaryKey(bookUuid: book.bookUuid, userUuid: user2.userUuid).genSelectExpect(db: con)

            XCTAssertEqual(userBook, new)
        }
    }
    
    func testDynamicUpdate() throws {
        let db = setupPool()
        var user = DbUser.random()

        try! db.write { con in
            try! user.genInsert(db: con)
            
            let assertUser: () -> () = {
                XCTAssertEqual(user, try! user.primaryKey().genSelectExpect(db: con))
            }
            
            user.bool = !user.bool
            
            try! user.primaryKey().genUpdateDynamic(db: con, columns: [user.createColumnBool()])
            
            assertUser()
            
            user.serializedInfoNullableAutoSet(serializedInfoNullable: nil)
            user.serializedInfoAutoSet(serializedInfo: .data("test"))
            
            try! user.primaryKey().genUpdateDynamic(db: con, columns: [user.createColumnSerializedInfo(), user.createColumnSerializedInfoNullable()])
            
            assertUser()
        }
    }
}
