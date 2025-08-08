import Foundation
import GRDB
import GRDBPerformance
import XCTest

class UpsertTest: XCTestCase {
    func testUpsert() {
        let db = setupPool()
        var user = DbUser.random()

        try! db.write { con in
            // First try to update it
            try! user.genInsert(db: con)

            user.integer += 1

            let assertUser: () -> Void = {
                XCTAssertEqual(user, try! user.primaryKey().genSelectExpect(db: con))
            }

            // Update the just-upserted column
            try! user.genUpsertDynamic(db: con, columns: [.integer])

            assertUser()

            // Let's try two columns
            user.bool = !user.bool
            user.jsonStructArray += [.init(age: 1)]

            try! user.genUpsertDynamic(db: con, columns: [.bool, .jsonStructArray])

            assertUser()

            // Upsert single column
            let upsertSingleColumn: (String?) -> Void = {
                user.firstName = $0

                try! user.genUpsertFirstName(db: con)

                assertUser()
            }

            upsertSingleColumn(nil)
            upsertSingleColumn("Something")

            // upsert a whole new user
            user.userUuid = UUID()

            try! user.genUpsertDynamic(db: con, columns: [.bool])

            assertUser()
        }
    }

    func testUpsertMutate() {
        let db = setupPool()
        var user = DbUser.random()

        try! db.write { con in
            let firstName = "SomeFirstName"

            try user.genUpsertDynamicMutate(db: con, columns: [.firstName(firstName)])

            XCTAssertEqual(firstName, user.firstName!)

            let updatedUser = try user.primaryKey().genSelectExpect(db: con)

            XCTAssertEqual(firstName, updatedUser.firstName!)
        }
    }
}
