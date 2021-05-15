import XCTest
import GRDBPerformance
import GRDB
import Foundation

class EqualsCheck: XCTestCase {
    func testEquals() {
        let db = setupPool()
        let dbUser = DbUser(userUuid: UUID(), firstName: "SomeName", jsonStruct: .init(age: 1), jsonStructOptional: nil, jsonStructArray: [.init(age: 2)], jsonStructArrayOptional: nil, integer: 4, bool: true, serializedInfo: .data("Something"), serializedInfoNullable: nil)

        try! dbUser.genInsert(dbWriter: db)

        let encoder = JSONEncoder()
        let dbUserData = try! encoder.encode(dbUser)

        let dbUserRetrieved = try! dbUser.primaryKey().genSelectExpect(dbReader: db)

        XCTAssertEqual(dbUserData, try! encoder.encode(dbUserRetrieved))

        try! dbUser.primaryKey().genDelete(dbWriter: db)

        let user = User(userUuid: dbUser.userUuid,
                firstName: dbUser.firstName,
                jsonStruct: dbUser.jsonStruct,
                jsonStructOptional: dbUser.jsonStructOptional,
                jsonStructArray: dbUser.jsonStructArray,
                jsonStructArrayOptional: dbUser.jsonStructArrayOptional,
                integer: dbUser.integer,
                bool: dbUser.bool,
                serializedInfo: dbUser.serializedInfo,
                serializedInfoNullable: dbUser.serializedInfoNullable)
        let userData = try! encoder.encode(user)

        XCTAssertEqual(dbUserData, userData)

        try! db.write { con in
            try! user.insert(con)

            let dbUserNew = try! DbUser.PrimaryKey(userUuid: user.userUuid).genSelectExpect(db: con)

            XCTAssertEqual(userData, try! encoder.encode(dbUserNew))
        }
    }
}
