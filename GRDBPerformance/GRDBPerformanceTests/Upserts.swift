import XCTest
import GRDBPerformance
import GRDB
import Foundation

class UpsertTest: XCTestCase {
    func testUpsert() {
        let db = setupPool()
        var user = DbUser(userUuid: UUID(), firstName: nil, jsonStruct: .init(age: 1), jsonStructOptional: nil, jsonStructArray: [], jsonStructArrayOptional: [], integer: 1)

        // First try to update it
        try! user.genInsert(dbWriter: db)

        user.integer += 1

        user.upsertExample(dbWriter: <#T##T##T#>)
    }
}