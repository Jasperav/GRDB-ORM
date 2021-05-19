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

class ReplaceTest: XCTestCase {
    func testReplace() throws {
        let db = setupPool()

        try! db.write { con in
            let checkCount: (Int) -> () = {
                let current = try! Int.fetchOne(con, sql: "select count(*) from user ")

                XCTAssertEqual($0, current)
            }

            checkCount(0)

            let createUser: () -> DbUser = {
                DbUser.random()
            }

            var user = createUser()

            try! user.genInsert(db: con)

            checkCount(1)

            user.firstName = "new"

            try! user.genReplace(db: con)

            checkCount(1)

            try! createUser().genInsert(db: con)

            checkCount(2)
        }
    }
}

class UpdatableColumnTest: XCTestCase {
    func testUpdatableColumn() {
        let db = setupPool()
        var user = DbUser.random()

        try! db.write { con in
            try! user.genInsert(db: con)
            
            let newFirstName = "new"
            
            try! user.primaryKey().genUpdateFirstName(db: con, firstName: newFirstName)

            user.firstName = newFirstName

            XCTAssertEqual(user, try! user.primaryKey().genSelectExpect(db: con))
        }
    }
}

class InsertOrIgnoreTest: XCTestCase {
    func test() {
        let db = setupPool()
        let user = DbUser.random()

        try! db.write { con in
            try! user.genInsert(db: con)

            var userClone = user

            // The first name should not be updated
            userClone.firstName = "somethingElse"

            try! userClone.genInsertOrIgnore(db: con)

            let retrieved = try! user.primaryKey().genSelectExpect(db: con)

            XCTAssertEqual(user, retrieved)
        }
    }
}
