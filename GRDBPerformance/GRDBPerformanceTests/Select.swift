import Foundation
import GRDB
import GRDBPerformance
import XCTest

class SelectPerformanceTest: XCTestCase {
    func testGenerated() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            // Don't call the expect function here to mimic the testGRDB method (both return User?)
            _ = try! DbUser.PrimaryKey(userUuid: uuid).genSelect(db: db)!
        })
    }

    func testGRDB() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            _ = try! User.fetchOne(db, key: uuid.uuidString)!
        })
    }

    func testSelectCount() throws {
        let db = setupPool()

        try db.write { con in
            XCTAssertEqual(0, try DbUser.genSelectCount(db: con))

            try DbUser.random().genInsert(db: con)

            XCTAssertEqual(1, try DbUser.genSelectCount(db: con))
        }
    }

    func testSelectExists() throws {
        let db = setupPool()

        try db.write { con in
            let user = DbUser.random()

            XCTAssertFalse(try user.primaryKey().genSelectExists(db: con))

            try user.genInsert(db: con)

            XCTAssert(try user.primaryKey().genSelectExists(db: con))
        }
    }

    func testSelectAll() throws {
        let db = setupPool()

        try db.write { con in
            let user = DbUser.random()

            try user.genInsert(db: con)

            let users = try DbUser.genSelectAll(db: con)

            XCTAssertEqual(1, users.count)
            XCTAssertEqual(user, users[0])
        }
    }
}
