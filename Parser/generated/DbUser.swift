// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbUser: FetchableRecord, PersistableRecord, Codable, Equatable {
    // Static queries
    public static let insertUniqueQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    public static let replaceUniqueQuery = "replace into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    public static let deleteAllQuery = "delete from User"
    public static let updateUniqueQuery = "update User set firstName = ?, jsonStruct = ?, jsonStructOptional = ?, jsonStructArray = ?, jsonStructArrayOptional = ?, integer = ?, bool = ?, serializedInfo = ?, serializedInfoNullable = ? where userUuid = ?"

    // Mapped columns to properties
    public let userUuid: UUID
    public var firstName: String?
    public var jsonStruct: JsonType
    public var jsonStructOptional: JsonType?
    public var jsonStructArray: [JsonType]
    public var jsonStructArrayOptional: [JsonType]?
    public var integer: Int
    public var bool: Bool
    public private(set) var serializedInfo: Data
    public func serializedInfoAutoConvert() -> SerializedInfo {
        try! SerializedInfo(serializedData: serializedInfo)
    }

    public mutating func serializedInfoAutoSet(serializedInfo: SerializedInfo) {
        self.serializedInfo = try! serializedInfo.serializedData()
    }

    public private(set) var serializedInfoNullable: Data?
    public func serializedInfoNullableAutoConvert() -> SerializedInfo? {
        guard let serializedInfoNullable = serializedInfoNullable else {
            return nil
        }
        return try! SerializedInfo(serializedData: serializedInfoNullable)
    }

    public mutating func serializedInfoNullableAutoSet(serializedInfoNullable: SerializedInfo?) {
        self.serializedInfoNullable = try! serializedInfoNullable?.serializedData()
    }

    // Default initializer
    public init(userUuid: UUID,
                firstName: String?,
                jsonStruct: JsonType,
                jsonStructOptional: JsonType?,
                jsonStructArray: [JsonType],
                jsonStructArrayOptional: [JsonType]?,
                integer: Int,
                bool: Bool,
                serializedInfo: SerializedInfo,
                serializedInfoNullable: SerializedInfo?)
    {
        self.userUuid = userUuid
        self.firstName = firstName
        self.jsonStruct = jsonStruct
        self.jsonStructOptional = jsonStructOptional
        self.jsonStructArray = jsonStructArray
        self.jsonStructArrayOptional = jsonStructArrayOptional
        self.integer = integer
        self.bool = bool
        self.serializedInfo = try! serializedInfo.serializedData()
        self.serializedInfoNullable = try! serializedInfoNullable?.serializedData()
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
        bool = row[7 + startingIndex]
        serializedInfo = row[8 + startingIndex]
        serializedInfoNullable = row[9 + startingIndex]
    }

    // The initializer defined by the protocol
    public init(row: Row) {
        self.init(row: row, startingIndex: 0)
    }

    // Easy way to get the PrimaryKey from the table
    public func primaryKey() -> DbUserPrimaryKey {
        .init(userUuid: userUuid)
    }

    public func genInsert(db: Database, assertOneRowAffected: Bool = true) throws {
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
            bool,
            try serializedInfo.serializedData(),
            try serializedInfoNullable?.serializedData(),
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
    }

    public func genInsert<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try genInsert(db: database)
        }
    }

    public func genReplace(db: Database, assertOneRowAffected: Bool = true) throws {
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
            bool,
            try serializedInfo.serializedData(),
            try serializedInfoNullable?.serializedData(),
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
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

    public func genUpdate(db: Database, assertOneRowAffected: Bool = true) throws {
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
            bool,
            try serializedInfo.serializedData(),
            try serializedInfoNullable?.serializedData(),
            userUuid.uuidString,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
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
    public func genDelete(db: Database, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.deleteQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genDelete<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try genDelete(db: database)
        }
    }

    public enum UpdatableColumn {
        case firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable

        public static let updateFirstNameQuery = "update User set firstName = ? where userUuid = ?"
        public static let updateJsonStructQuery = "update User set jsonStruct = ? where userUuid = ?"
        public static let updateJsonStructOptionalQuery = "update User set jsonStructOptional = ? where userUuid = ?"
        public static let updateJsonStructArrayQuery = "update User set jsonStructArray = ? where userUuid = ?"
        public static let updateJsonStructArrayOptionalQuery = "update User set jsonStructArrayOptional = ? where userUuid = ?"
        public static let updateIntegerQuery = "update User set integer = ? where userUuid = ?"
        public static let updateBoolQuery = "update User set bool = ? where userUuid = ?"
        public static let updateSerializedInfoQuery = "update User set serializedInfo = ? where userUuid = ?"
        public static let updateSerializedInfoNullableQuery = "update User set serializedInfoNullable = ? where userUuid = ?"
    }

    public func genUpdateFirstName(db: Database, firstName: String?, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            firstName,
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateFirstNameQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateFirstName<T: DatabaseWriter>(dbWriter: T, firstName: String?) throws {
        try dbWriter.write { database in
            try genUpdateFirstName(db: database, firstName: firstName)
        }
    }

    public func genUpdateJsonStruct(db: Database, jsonStruct: JsonType, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateJsonStructQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateJsonStruct<T: DatabaseWriter>(dbWriter: T, jsonStruct: JsonType) throws {
        try dbWriter.write { database in
            try genUpdateJsonStruct(db: database, jsonStruct: jsonStruct)
        }
    }

    public func genUpdateJsonStructOptional(db: Database, jsonStructOptional: JsonType?, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateJsonStructOptionalQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateJsonStructOptional<T: DatabaseWriter>(dbWriter: T, jsonStructOptional: JsonType?) throws {
        try dbWriter.write { database in
            try genUpdateJsonStructOptional(db: database, jsonStructOptional: jsonStructOptional)
        }
    }

    public func genUpdateJsonStructArray(db: Database, jsonStructArray: [JsonType], assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateJsonStructArrayQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateJsonStructArray<T: DatabaseWriter>(dbWriter: T, jsonStructArray: [JsonType]) throws {
        try dbWriter.write { database in
            try genUpdateJsonStructArray(db: database, jsonStructArray: jsonStructArray)
        }
    }

    public func genUpdateJsonStructArrayOptional(db: Database, jsonStructArrayOptional: [JsonType]?, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateJsonStructArrayOptionalQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateJsonStructArrayOptional<T: DatabaseWriter>(dbWriter: T, jsonStructArrayOptional: [JsonType]?) throws {
        try dbWriter.write { database in
            try genUpdateJsonStructArrayOptional(db: database, jsonStructArrayOptional: jsonStructArrayOptional)
        }
    }

    public func genUpdateInteger(db: Database, integer: Int, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            integer,
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateIntegerQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateInteger<T: DatabaseWriter>(dbWriter: T, integer: Int) throws {
        try dbWriter.write { database in
            try genUpdateInteger(db: database, integer: integer)
        }
    }

    public func genUpdateBool(db: Database, bool: Bool, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            bool,
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateBoolQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateBool<T: DatabaseWriter>(dbWriter: T, bool: Bool) throws {
        try dbWriter.write { database in
            try genUpdateBool(db: database, bool: bool)
        }
    }

    public func genUpdateSerializedInfo(db: Database, serializedInfo: SerializedInfo, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            try serializedInfo.serializedData(),
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateSerializedInfoQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateSerializedInfo<T: DatabaseWriter>(dbWriter: T, serializedInfo: SerializedInfo) throws {
        try dbWriter.write { database in
            try genUpdateSerializedInfo(db: database, serializedInfo: serializedInfo)
        }
    }

    public func genUpdateSerializedInfoNullable(db: Database, serializedInfoNullable: SerializedInfo?, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            try serializedInfoNullable?.serializedData(),
            userUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateSerializedInfoNullableQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    public func genUpdateSerializedInfoNullable<T: DatabaseWriter>(dbWriter: T, serializedInfoNullable: SerializedInfo?) throws {
        try dbWriter.write { database in
            try genUpdateSerializedInfoNullable(db: database, serializedInfoNullable: serializedInfoNullable)
        }
    }
}
