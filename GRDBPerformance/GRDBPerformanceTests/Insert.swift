import XCTest
import GRDBPerformance
import GRDB

class InsertPerformanceTest: XCTestCase {
    func testGenerated() throws {
        startMeasure(block: { db in
            try! DbUser.random().genInsert(db: db)
        })
    }
    
    func testGRDB() throws {
        startMeasure(block: { db in
            try! User.random().insert(db)
        })
    }

    func startMeasure(block: (Database) -> ()) {
        self.measure {
            let db = setupPool()
            
            try! db.write { db in
                for _ in 0...amountToGenerate {
                    block(db)
                }
            }
        }
    }
}
