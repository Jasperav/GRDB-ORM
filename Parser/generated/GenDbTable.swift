// // This file is generated, do not edit

import Foundation
import GRDB

public
protocol GenDbTable {
    static func genSelectCount(db: Database) throws -> Int
    static func genDeleteAll(db: Database) throws
}
