// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbUser: FetchableRecord, PersistableRecord, Codable, Equatable {
    // Static queries
    public static let insertUniqueQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    public static let replaceUniqueQuery = "replace into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    public static let insertOrIgnoreUniqueQuery = "insert or ignore into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
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
    public func primaryKey() -> PrimaryKey {
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

    public func genInsertOrIgnore(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.insertOrIgnoreUniqueQuery)

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
            bool,
            try serializedInfo.serializedData(),
            try serializedInfoNullable?.serializedData(),
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public static func genDeleteAll(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.deleteAllQuery)

        try statement.execute()
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

    // Write the primary key struct, useful for selecting or deleting a unique row
    public struct PrimaryKey {
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

        // Same as function 'genSelectUnique', but throws an error when no record has been found
        public func genSelectExpect(db: Database) throws -> DbUser {
            if let instance = try genSelect(db: db) {
                return instance
            } else {
                throw DatabaseError(message: "Didn't found a record for \(self)")
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

        public enum UpdatableColumn {
            case userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable

            public static let updateUserUuidQuery = "update User set userUuid = ? where userUuid = ?"
            public static let updateFirstNameQuery = "update User set firstName = ? where userUuid = ?"
            public static let updateJsonStructQuery = "update User set jsonStruct = ? where userUuid = ?"
            public static let updateJsonStructOptionalQuery = "update User set jsonStructOptional = ? where userUuid = ?"
            public static let updateJsonStructArrayQuery = "update User set jsonStructArray = ? where userUuid = ?"
            public static let updateJsonStructArrayOptionalQuery = "update User set jsonStructArrayOptional = ? where userUuid = ?"
            public static let updateIntegerQuery = "update User set integer = ? where userUuid = ?"
            public static let updateBoolQuery = "update User set bool = ? where userUuid = ?"
            public static let updateSerializedInfoQuery = "update User set serializedInfo = ? where userUuid = ?"
            public static let updateSerializedInfoNullableQuery = "update User set serializedInfoNullable = ? where userUuid = ?"

            public static let upsertFirstNameQuery = "update User set firstName = ? where userUuid = ? on conflict(userUuid) do update set firstName=excluded.firstName"
            public static let upsertJsonStructQuery = "update User set jsonStruct = ? where userUuid = ? on conflict(userUuid) do update set jsonStruct=excluded.jsonStruct"
            public static let upsertJsonStructOptionalQuery = "update User set jsonStructOptional = ? where userUuid = ? on conflict(userUuid) do update set jsonStructOptional=excluded.jsonStructOptional"
            public static let upsertJsonStructArrayQuery = "update User set jsonStructArray = ? where userUuid = ? on conflict(userUuid) do update set jsonStructArray=excluded.jsonStructArray"
            public static let upsertJsonStructArrayOptionalQuery = "update User set jsonStructArrayOptional = ? where userUuid = ? on conflict(userUuid) do update set jsonStructArrayOptional=excluded.jsonStructArrayOptional"
            public static let upsertIntegerQuery = "update User set integer = ? where userUuid = ? on conflict(userUuid) do update set integer=excluded.integer"
            public static let upsertBoolQuery = "update User set bool = ? where userUuid = ? on conflict(userUuid) do update set bool=excluded.bool"
            public static let upsertSerializedInfoQuery = "update User set serializedInfo = ? where userUuid = ? on conflict(userUuid) do update set serializedInfo=excluded.serializedInfo"
            public static let upsertSerializedInfoNullableQuery = "update User set serializedInfoNullable = ? where userUuid = ? on conflict(userUuid) do update set serializedInfoNullable=excluded.serializedInfoNullable"
        }

        public func genUpdateUserUuid(db: Database, userUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                userUuid.uuidString,
                self.userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateUserUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateFirstName(db: Database, firstName: String?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                firstName,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateFirstNameQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertFirstName(db: Database, firstName: String?) throws {
            let arguments: StatementArguments = try [
                firstName,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertFirstNameQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
        }

        public func genUpdateJsonStruct(db: Database, jsonStruct: JsonType, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                {
                    let data = try Shared.jsonEncoder.encode(jsonStruct)
                    return String(data: data, encoding: .utf8)!
                }(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateJsonStructQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertJsonStruct(db: Database, jsonStruct: JsonType) throws {
            let arguments: StatementArguments = try [
                {
                    let data = try Shared.jsonEncoder.encode(jsonStruct)
                    return String(data: data, encoding: .utf8)!
                }(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertJsonStructQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
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

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateJsonStructOptionalQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertJsonStructOptional(db: Database, jsonStructOptional: JsonType?) throws {
            let arguments: StatementArguments = try [
                {
                    try jsonStructOptional.map {
                        let data = try Shared.jsonEncoder.encode($0)
                        return String(data: data, encoding: .utf8)!
                    }
                }(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertJsonStructOptionalQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
        }

        public func genUpdateJsonStructArray(db: Database, jsonStructArray: [JsonType], assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                {
                    let data = try Shared.jsonEncoder.encode(jsonStructArray)
                    return String(data: data, encoding: .utf8)!
                }(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateJsonStructArrayQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertJsonStructArray(db: Database, jsonStructArray: [JsonType]) throws {
            let arguments: StatementArguments = try [
                {
                    let data = try Shared.jsonEncoder.encode(jsonStructArray)
                    return String(data: data, encoding: .utf8)!
                }(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertJsonStructArrayQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
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

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateJsonStructArrayOptionalQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertJsonStructArrayOptional(db: Database, jsonStructArrayOptional: [JsonType]?) throws {
            let arguments: StatementArguments = try [
                {
                    try jsonStructArrayOptional.map {
                        let data = try Shared.jsonEncoder.encode($0)
                        return String(data: data, encoding: .utf8)!
                    }
                }(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertJsonStructArrayOptionalQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
        }

        public func genUpdateInteger(db: Database, integer: Int, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                integer,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateIntegerQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertInteger(db: Database, integer: Int) throws {
            let arguments: StatementArguments = try [
                integer,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertIntegerQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
        }

        public func genUpdateBool(db: Database, bool: Bool, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                bool,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateBoolQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertBool(db: Database, bool: Bool) throws {
            let arguments: StatementArguments = try [
                bool,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertBoolQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
        }

        public func genUpdateSerializedInfo(db: Database, serializedInfo: SerializedInfo, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try serializedInfo.serializedData(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateSerializedInfoQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertSerializedInfo(db: Database, serializedInfo: SerializedInfo) throws {
            let arguments: StatementArguments = try [
                try serializedInfo.serializedData(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertSerializedInfoQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
        }

        public func genUpdateSerializedInfoNullable(db: Database, serializedInfoNullable: SerializedInfo?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try serializedInfoNullable?.serializedData(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateSerializedInfoNullableQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpsertSerializedInfoNullable(db: Database, serializedInfoNullable: SerializedInfo?) throws {
            let arguments: StatementArguments = try [
                try serializedInfoNullable?.serializedData(),
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.upsertSerializedInfoNullableQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()
        }
    }
}
