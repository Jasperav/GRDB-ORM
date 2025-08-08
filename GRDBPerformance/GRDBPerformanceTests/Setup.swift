import Foundation
import GRDB
import GRDBPerformance
import XCTest

let amountToGenerate = 10000

enum TestRunner {
    static func setupDb() -> (DatabaseQueue, [UUID]) {
        let db = setupPool()
        var uuids = [UUID]()

        try! db.write { db in
            for _ in 0 ..< amountToGenerate {
                let user = DbUser.random()

                uuids.append(user.userUuid)

                try! user.genInsert(db: db)
            }
        }

        return (db, uuids)
    }

    static func startMeasure(theTest: XCTestCase, block: (Database, UUID) -> Void) {
        theTest.measure {
            let (db, uuids) = setupDb()

            try! db.write { db in
                for uuid in uuids {
                    block(db, uuid)
                }
            }
        }
    }
}
