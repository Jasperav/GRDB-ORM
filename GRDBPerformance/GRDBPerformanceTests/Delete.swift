import Foundation
import GRDB
import GRDBPerformance
import XCTest

class DeletePerformanceTest: XCTestCase {
    func testGenerated() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! DbUser.PrimaryKey(userUuid: uuid).genDelete(db: db)
        })
    }

    func testGRDB() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! User.deleteOne(db, key: uuid.uuidString)
        })
    }

    func testDeleteBy() {
        let db = setupPool()

        try! db.write { con in
            var user0 = DbUser.random()
            var user1 = DbUser.random()
            var user2 = DbUser.random()

            user0.firstName = "hi"
            user1.firstName = user0.firstName
            user2.firstName = user0.firstName! + "extra"

            try user0.genInsert(db: con)
            try user1.genInsert(db: con)
            try user2.genInsert(db: con)

            try DbUser.genDeleteByFirstName(db: con, firstName: user0.firstName!)

            XCTAssertEqual(2, con.changesCount)
            XCTAssertNil(try user0.primaryKey().genSelect(db: con))
            XCTAssertNotNil(try user2.primaryKey().genSelect(db: con))
        }
    }
}
