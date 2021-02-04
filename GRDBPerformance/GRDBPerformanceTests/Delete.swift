import XCTest
import GRDBPerformance
import GRDB
import Foundation

class DeletePerformanceTest: XCTestCase {
    func testGenerated() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! DbUserPrimaryKey(userUuid: uuid).genDelete(db: db)
        })
    }

    func testGRDB() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! User.deleteOne(db, key: uuid.uuidString)
        })
    }
}
