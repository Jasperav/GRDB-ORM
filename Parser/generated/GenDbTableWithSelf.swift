// // This file is generated, do not edit

import Foundation
import GRDB

public
protocol GenDbTableWithSelf: GenDbTable {
    associatedtype Table

    static func genSelectAll(db: Database) throws -> [Table]
}
