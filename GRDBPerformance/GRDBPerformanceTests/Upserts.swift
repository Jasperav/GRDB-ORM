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
            
            let assertUser: () -> () = {
                XCTAssertEqual(user, try! user.primaryKey().genSelectExpect(db: con))
            }

            // Update the just-upserted column
            try! user.upsert(db: con, columns: [.integer])

            assertUser()
            
            // Let's try two columns
            user.bool = !user.bool
            user.jsonStructArray += [.init(age: 1)]
            
            try! user.upsert(db: con, columns: [.bool, .jsonStructArray])
            
            assertUser()

            // Upsert single column
            let upsertSingleColumn: (String?) -> () = {
                user.firstName = $0

                try! user.genUpsertFirstName(db: con)

                assertUser()
            }

            upsertSingleColumn(nil)
            upsertSingleColumn("Something")
            
            // upsert a whole new user
            user.userUuid = UUID()
            
            try! user.upsert(db: con, columns: [.bool])
            
            assertUser()
        }
    }
}
