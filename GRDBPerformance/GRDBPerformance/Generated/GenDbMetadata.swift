// // This file is generated, do not edit

import Foundation
import GRDB

public enum GenDbMetadata {
    public static func tables() -> [GenDbTable.Type] {
        [DbBook.self, DbUser.self, DbUserBook.self]
    }
}