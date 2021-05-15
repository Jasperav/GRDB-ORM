import XCTest
import GRDBPerformance
import GRDB
import Foundation

class SelectPerformanceTest: XCTestCase {
    func testGenerated() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            // Don't call the expect function here to mimic the testGRDB method (both return User?)
            let _ = try! DbUser.PrimaryKey(userUuid: uuid).genSelect(db: db)!
        })
    }

    func testGRDB() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            let _ = try! User.fetchOne(db, key: uuid.uuidString)!
        })
    }
}
