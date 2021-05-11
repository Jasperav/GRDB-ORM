import XCTest
import GRDBPerformance
import GRDB
import Foundation

class UpsertTest: XCTestCase {
    func testUpsert() {
        let db = setupPool()
        var user = DbUser.random()

        // First try to update it
        try! user.genInsert(dbWriter: db)

        user.integer += 1

        try! user.upsertExample(dbWriter: db)

        let retrievedUser = try! user.primaryKey().genSelectExpect(dbReader: db)

        XCTAssertEqual(user, retrievedUser)
    }
}
