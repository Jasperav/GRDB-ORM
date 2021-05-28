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

            // Upsert the whole user
            try! user.upsertExample(db: con)

            let assertUser: () -> () = {
                XCTAssertEqual(user, try! user.primaryKey().genSelectExpect(db: con))
            }

            assertUser()

            // Upsert single column
            let upsertSingleColumn: (String?) -> () = {
                user.firstName = $0

                try! user.primaryKey().genUpsertFirstName(db: db, firstName: user.firstName)

                assertUser()
            }

            upsertSingleColumn(nil)
            upsertSingleColumn("Something")
        }
    }
}
