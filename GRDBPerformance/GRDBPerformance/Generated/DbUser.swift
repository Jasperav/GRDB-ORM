// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbUser: FetchableRecord, PersistableRecord, Codable {
    // Static queries
    public static let insertUniqueQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer) values (?, ?, ?, ?, ?, ?, ?)"
    public static let replaceUniqueQuery = "replace into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer) values (?, ?, ?, ?, ?, ?, ?)"
    public static let deleteAllQuery = "delete from User"
    public static let updateUniqueQuery = "update User set firstName = ?, jsonStruct = ?, jsonStructOptional = ?, jsonStructArray = ?, jsonStructArrayOptional = ?, integer = ? where userUuid = ?"

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
        let statement = try db.cachedUpdateStatement(sql: Self.insertUniqueQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        // Only 1 row should be affected
        assert(db.changesCount == 1)
    }

    public func genInsert<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try genInsert(db: database)
        }
    }

    public func genReplace(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.replaceUniqueQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        // Only 1 row should be affected
        assert(db.changesCount == 1)
    }

    public func genReplace<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try genReplace(db: database)
        }
    }

    public static func genDeleteAll(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.deleteAllQuery)

        try statement.execute()
    }

    public static func genDeleteAll<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try genDeleteAll(db: database)
        }
    }

    public func genUpdate(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.updateUniqueQuery)

        let arguments: StatementArguments = try [
            firstName,
            {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            userUuid.uuidString,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        // Only 1 row should be affected
        assert(db.changesCount == 1)
    }

    public func genUpdate<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try genUpdate(db: database)
        }
    }
}

// Write the primary key struct, useful for selecting or deleting a unique row
public struct DbUserPrimaryKey {
    // Static queries
    public static let selectQuery = "select * from User where userUuid = ?"
    public static let deleteQuery = "delete from User where userUuid = ?"

    // Mapped columns to properties
    public let userUuid: UUID

    // Default initializer
    public init(userUuid: UUID) {
        self.userUuid = userUuid
    }

    // Queries a unique row in the database, the row may or may not exist
    public func genSelect(db: Database) throws -> DbUser? {
        let arguments: StatementArguments = try [
            userUuid.uuidString,
        ]

        let statement = try db.cachedSelectStatement(sql: Self.selectQuery)

        statement.setUncheckedArguments(arguments)

        return try DbUser.fetchOne(statement)
    }

    public func genSelect<T: DatabaseReader>(dbReader: T) throws -> DbUser? {
        try dbReader.read { database in
            try genSelect(db: database)
        }
    }

    // Same as function 'genSelectUnique', but throws an error when no record has been found
    public func genSelectExpect(db: Database) throws -> DbUser {
        if let instance = try genSelect(db: db) {
            return instance
        } else {
            throw DatabaseError(message: "Didn't found a record for \(self)")
        }
    }

    public func genSelectExpect<T: DatabaseReader>(dbReader: T) throws -> DbUser {
        try dbReader.read { database in
            try genSelectExpect(db: database)
        }
    }

    // Deletes a unique row, asserts that the row actually existed
    public func genDelete(db: Database) throws {
        let arguments: StatementArguments = try [
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.deleteQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        assert(db.changesCount == 1)
    }

    public func genDelete<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try genDelete(db: database)
        }
    }
}

// Easy way to get the PrimaryKey from the table
public extension DbUser {
    func primary_key() -> DbUserPrimaryKey {
        .init(userUuid: userUuid)
    }
}
