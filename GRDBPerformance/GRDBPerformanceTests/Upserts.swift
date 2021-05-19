import XCTest
import GRDBPerformance
import GRDB
import Foundation

class UpsertTest: XCTestCase {
    func testUpsert() {
        let db = setupPool()
        var user = DbUser.random()

        try! db.write { con in
            // First try to update it
            try! user.genInsert(db: con)

            user.integer += 1

            try! user.upsertExample(db: con)

            let retrievedUser = try! user.primaryKey().genSelectExpect(db: con)

            XCTAssertEqual(user, retrievedUser)
        }
    }
}
