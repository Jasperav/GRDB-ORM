// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbUser: FetchableRecord, PersistableRecord, Codable {
    // Static queries
    public static let insert_unique_query = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer) values (?, ?, ?, ?, ?, ?, ?)"
    public static let update_unique_query = "update User set firstName = ?, jsonStruct = ?, jsonStructOptional = ?, jsonStructArray = ?, jsonStructArrayOptional = ?, integer = ? where userUuid = ?"

    // Mapped columns to properties
    public let userUuid: UUID
    public var firstName: String?
    public var jsonStruct: JsonType
    public var jsonStructOptional: JsonType?
    public var jsonStructArray: [JsonType]
    public var jsonStructArrayOptional: [JsonType]?
    public var integer: Int

    // Default initializer
    public init(userUuid: UUID,
                firstName: String?,
                jsonStruct: JsonType,
                jsonStructOptional: JsonType?,
                jsonStructArray: [JsonType],
                jsonStructArrayOptional: [JsonType]?,
                integer: Int)
    {
        self.userUuid = userUuid
        self.firstName = firstName
        self.jsonStruct = jsonStruct
        self.jsonStructOptional = jsonStructOptional
        self.jsonStructArray = jsonStructArray
        self.jsonStructArrayOptional = jsonStructArrayOptional
        self.integer = integer
    }

    // Row initializer
    public init(row: Row, startingIndex: Int) {
        userUuid = row[0 + startingIndex]
        firstName = row[1 + startingIndex]
        jsonStruct = try! Shared.jsonDecoder.decode(JsonType.self, from: row[2 + startingIndex])
        jsonStructOptional = {
            if row.hasNull(atIndex: 3 + startingIndex) {
                return nil
            } else {
                return try! Shared.jsonDecoder.decode(JsonType.self, from: row[3 + startingIndex])
            }
        }()
        jsonStructArray = try! Shared.jsonDecoder.decode([JsonType].self, from: row[4 + startingIndex])
        jsonStructArrayOptional = {
            if row.hasNull(atIndex: 5 + startingIndex) {
                return nil
            } else {
                return try! Shared.jsonDecoder.decode([JsonType].self, from: row[5 + startingIndex])
            }
        }()
        integer = row[6 + startingIndex]
    }

    // The initializer defined by the protocol
    public init(row: Row) {
        self.init(row: row, startingIndex: 0)
    }

    public func genInsert(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.insert_unique_query)
        let values = [
            userUuid.uuidString.databaseValue,
            firstName?.databaseValue ?? .null,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                let string = String(data: data, encoding: .utf8)!

                return string.databaseValue
            }(),
            try {
                if let jsonStructOptional = jsonStructOptional {
                    let data = try Shared.jsonEncoder.encode(jsonStructOptional)
                    let string = String(data: data, encoding: .utf8)!

                    return string.databaseValue
                } else {
                    return DatabaseValue.null
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                let string = String(data: data, encoding: .utf8)!

                return string.databaseValue
            }(),
            try {
                if let jsonStructArrayOptional = jsonStructArrayOptional {
                    let data = try Shared.jsonEncoder.encode(jsonStructArrayOptional)
                    let string = String(data: data, encoding: .utf8)!

                    return string.databaseValue
                } else {
                    return DatabaseValue.null
                }
            }(),
            integer.databaseValue,
        ]

        statement.setUncheckedArguments(StatementArguments(values: values))

        try statement.execute()

        // Only 1 row should be affected
        assert(db.changesCount == 1)
    }

    public func genUpdate(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.update_unique_query)
        let values = [
            firstName?.databaseValue ?? .null,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                let string = String(data: data, encoding: .utf8)!

                return string.databaseValue
            }(),
            try {
                if let jsonStructOptional = jsonStructOptional {
                    let data = try Shared.jsonEncoder.encode(jsonStructOptional)
                    let string = String(data: data, encoding: .utf8)!

                    return string.databaseValue
                } else {
                    return DatabaseValue.null
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                let string = String(data: data, encoding: .utf8)!

                return string.databaseValue
            }(),
            try {
                if let jsonStructArrayOptional = jsonStructArrayOptional {
                    let data = try Shared.jsonEncoder.encode(jsonStructArrayOptional)
                    let string = String(data: data, encoding: .utf8)!

                    return string.databaseValue
                } else {
                    return DatabaseValue.null
                }
            }(),
            integer.databaseValue,
            userUuid.uuidString.databaseValue,
        ]

        statement.setUncheckedArguments(StatementArguments(values: values))

        try statement.execute()

        // Only 1 row should be affected
        assert(db.changesCount == 1)
    }
}

// Write the primary key struct, useful for selecting or deleting a unique row
public struct DbUserPrimaryKey {
    // Static queries
    public static let select_query = "select * from User where userUuid = ?"
    public static let delete_query = "delete from User where userUuid = ?"

    // Mapped columns to properties
    public let userUuid: UUID

    // Default initializer
    public init(userUuid: UUID) {
        self.userUuid = userUuid
    }

    // Queries a unique row in the database, the row may or may not exist
    public func genSelect(db: Database) throws -> DbUser? {
        let statement = try db.cachedSelectStatement(sql: Self.select_query)

        statement.setUncheckedArguments(StatementArguments(values: [
            userUuid.uuidString.databaseValue,
        ]))

        return try DbUser.fetchOne(statement)
    }

    // Same as function 'genSelectUnique', but throws an error when no record has been found
    public func genSelectExpect(db: Database) throws -> DbUser {
        if let instance = try genSelect(db: db) {
            return instance
        } else {
            throw DatabaseError(message: "Didn't found a record for \(self)")
        }
    }

    // Deletes a unique row, asserts that the row actually existed
    public func genDelete(db: Database) throws {
        let values = [
            userUuid.uuidString.databaseValue,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.delete_query)

        statement.setUncheckedArguments(StatementArguments(values: values))

        try statement.execute()

        assert(db.changesCount == 1)
    }
}

// Easy way to get the PrimaryKey from the table
public extension DbUser {
    func primary_key() -> DbUserPrimaryKey {
        .init(userUuid: userUuid)
    }
}
