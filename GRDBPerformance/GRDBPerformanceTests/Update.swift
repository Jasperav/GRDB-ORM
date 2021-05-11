import XCTest
import GRDBPerformance
import GRDB
import Foundation

class UpdatePerformanceTest: XCTestCase {
    func testGenerated() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! DbUser(userUuid: uuid,
                    firstName: nil,
                    jsonStruct: .init(age: 1),
                    jsonStructOptional: nil,
                    jsonStructArray: [],
                    jsonStructArrayOptional: nil,
                    integer: 1,
                    bool: true,
                    serializedInfo: .data("r"),
                    serializedInfoNullable: nil)
                    .genUpdate(db: db)
        })
    }

    func testGRDB() throws {
        TestRunner.startMeasure(theTest: self, block: { db, uuid in
            try! User(userUuid: uuid,
                    firstName: nil,
                    jsonStruct: .init(age: 1),
                    jsonStructOptional: nil,
                    jsonStructArray: [],
                    jsonStructArrayOptional: nil,
                    integer: 1,
                    bool: true,
                    serializedInfo: Data(),
                    serializedInfoNullable: nil)
                    .update(db)
        })
    }
}
