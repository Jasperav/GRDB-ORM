// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbUser: FetchableRecord, PersistableRecord, Codable, Equatable, Hashable, GenDbTable, GenDbTableWithSelf {
    // Static queries
    public static let insertUniqueQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    public static let replaceUniqueQuery = "replace into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    public static let insertOrIgnoreUniqueQuery = "insert or ignore into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    public static let deleteAllQuery = "delete from User"
    public static let selectAllQuery = "select userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable from User"
    public static let selectCountQuery = "select count(*) from User"
    public static let updateUniqueQuery = "update User set firstName = ?, jsonStruct = ?, jsonStructOptional = ?, jsonStructArray = ?, jsonStructArrayOptional = ?, integer = ?, bool = ?, serializedInfo = ?, serializedInfoNullable = ? where userUuid = ?"
    public static let upsertFirstNameQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set firstName=excluded.firstName"
    public static let upsertJsonStructQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set jsonStruct=excluded.jsonStruct"
    public static let upsertJsonStructOptionalQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set jsonStructOptional=excluded.jsonStructOptional"
    public static let upsertJsonStructArrayQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set jsonStructArray=excluded.jsonStructArray"
    public static let upsertJsonStructArrayOptionalQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set jsonStructArrayOptional=excluded.jsonStructArrayOptional"
    public static let upsertIntegerQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set integer=excluded.integer"
    public static let upsertBoolQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set bool=excluded.bool"
    public static let upsertSerializedInfoQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set serializedInfo=excluded.serializedInfo"
    public static let upsertSerializedInfoNullableQuery = "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set serializedInfoNullable=excluded.serializedInfoNullable"

    // Mapped columns to properties
    public var userUuid: UUID
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
                serializedInfoNullable: SerializedInfo?) {
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

    public func hash(into hasher: inout Hasher) {
        hasher.combine(userUuid)
    }

    public func genInsert(db: Database, assertOneRowAffected: Bool = true) throws {
        let statement = try db.cachedStatement(sql: Self.insertUniqueQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.insertUniqueQuery, statementArguments: arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
    }

    public func genInsertOrIgnore(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.insertOrIgnoreUniqueQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.insertOrIgnoreUniqueQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genReplace(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.replaceUniqueQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.replaceUniqueQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertFirstName(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertFirstNameQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertFirstNameQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertJsonStruct(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertJsonStructQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertJsonStructQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertJsonStructOptional(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertJsonStructOptionalQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertJsonStructOptionalQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertJsonStructArray(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertJsonStructArrayQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertJsonStructArrayQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertJsonStructArrayOptional(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertJsonStructArrayOptionalQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertJsonStructArrayOptionalQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertInteger(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertIntegerQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertIntegerQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertBool(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertBoolQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertBoolQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertSerializedInfo(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertSerializedInfoQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertSerializedInfoQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genUpsertSerializedInfoNullable(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.upsertSerializedInfoNullableQuery)

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.upsertSerializedInfoNullableQuery, statementArguments: arguments)

        try statement.execute()
    }

    public func genInsertOrDelete(db: Database, insert: Bool, assertOneRowAffected: Bool = true) throws {
        if insert {
            try genInsert(db: db, assertOneRowAffected: assertOneRowAffected)
        } else {
            try primaryKey().genDelete(db: db, assertOneRowAffected: assertOneRowAffected)
        }
    }

    public static func genDeleteAll(db: Database) throws {
        let statement = try db.cachedStatement(sql: Self.deleteAllQuery)

        Logging.log(Self.deleteAllQuery, statementArguments: .init())

        try statement.execute()
    }

    public
    static func genDeleteByUserUuid(db: Database, userUuid: UUID) throws {
        let arguments: StatementArguments = try [
            userUuid.uuidString
        ]

        Logging.log("delete from User where userUuid = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where userUuid = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteByFirstName(db: Database, firstName: String) throws {
        let arguments: StatementArguments = try [
            firstName
        ]

        Logging.log("delete from User where firstName = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where firstName = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteByJsonStruct(db: Database, jsonStruct: JsonType) throws {
        let arguments: StatementArguments = try [
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }()
        ]

        Logging.log("delete from User where jsonStruct = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where jsonStruct = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteByJsonStructOptional(db: Database, jsonStructOptional: JsonType) throws {
        let arguments: StatementArguments = try [
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructOptional)
                return String(data: data, encoding: .utf8)!
            }()
        ]

        Logging.log("delete from User where jsonStructOptional = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where jsonStructOptional = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteByJsonStructArray(db: Database, jsonStructArray: [JsonType]) throws {
        let arguments: StatementArguments = try [
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }()
        ]

        Logging.log("delete from User where jsonStructArray = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where jsonStructArray = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteByJsonStructArrayOptional(db: Database, jsonStructArrayOptional: [JsonType]) throws {
        let arguments: StatementArguments = try [
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArrayOptional)
                return String(data: data, encoding: .utf8)!
            }()
        ]

        Logging.log("delete from User where jsonStructArrayOptional = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where jsonStructArrayOptional = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteByInteger(db: Database, integer: Int) throws {
        let arguments: StatementArguments = try [
            integer
        ]

        Logging.log("delete from User where integer = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where integer = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteByBool(db: Database, bool: Bool) throws {
        let arguments: StatementArguments = try [
            bool
        ]

        Logging.log("delete from User where bool = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where bool = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteBySerializedInfo(db: Database, serializedInfo: SerializedInfo) throws {
        let arguments: StatementArguments = try [
            try! serializedInfo.serializedData()
        ]

        Logging.log("delete from User where serializedInfo = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where serializedInfo = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genDeleteBySerializedInfoNullable(db: Database, serializedInfoNullable: SerializedInfo) throws {
        let arguments: StatementArguments = try [
            try! serializedInfoNullable.serializedData()
        ]

        Logging.log("delete from User where serializedInfoNullable = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "delete from User where serializedInfoNullable = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genUpdate(db: Database, assertOneRowAffected: Bool = true) throws {
        let statement = try db.cachedStatement(sql: Self.updateUniqueQuery)

        let arguments: StatementArguments = try [
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData(),
            userUuid.uuidString
        ]

        statement.setUncheckedArguments(arguments)

        Logging.log(Self.updateUniqueQuery, statementArguments: arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
    }

    public enum UpdatableColumn: String {
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
    }

    public enum UpdatableColumnWithValue {
        case userUuid(UUID), firstName(String?), jsonStruct(JsonType), jsonStructOptional(JsonType?), jsonStructArray([JsonType]), jsonStructArrayOptional([JsonType]?), integer(Int), bool(Bool), serializedInfo(SerializedInfo), serializedInfoNullable(SerializedInfo?)

        var columnName: String {
            switch self {
            case .userUuid: return "userUuid"
            case .firstName: return "firstName"
            case .jsonStruct: return "jsonStruct"
            case .jsonStructOptional: return "jsonStructOptional"
            case .jsonStructArray: return "jsonStructArray"
            case .jsonStructArrayOptional: return "jsonStructArrayOptional"
            case .integer: return "integer"
            case .bool: return "bool"
            case .serializedInfo: return "serializedInfo"
            case .serializedInfoNullable: return "serializedInfoNullable"
            }
        }

        public func toUpdatableColumn() -> UpdatableColumn {
            switch self {
            case .userUuid: return .userUuid
            case .firstName: return .firstName
            case .jsonStruct: return .jsonStruct
            case .jsonStructOptional: return .jsonStructOptional
            case .jsonStructArray: return .jsonStructArray
            case .jsonStructArrayOptional: return .jsonStructArrayOptional
            case .integer: return .integer
            case .bool: return .bool
            case .serializedInfo: return .serializedInfo
            case .serializedInfoNullable: return .serializedInfoNullable
            }
        }

        public func update(entity: inout DbUser) {
            switch self {
            case let .userUuid(value): entity.userUuid = value
            case let .firstName(value): entity.firstName = value
            case let .jsonStruct(value): entity.jsonStruct = value
            case let .jsonStructOptional(value): entity.jsonStructOptional = value
            case let .jsonStructArray(value): entity.jsonStructArray = value
            case let .jsonStructArrayOptional(value): entity.jsonStructArrayOptional = value
            case let .integer(value): entity.integer = value
            case let .bool(value): entity.bool = value
            case let .serializedInfo(value): entity.serializedInfo = try! entity.serializedInfo.serializedData()
            case let .serializedInfoNullable(value): entity.serializedInfoNullable = try! entity.serializedInfoNullable?.serializedData()
            }
        }
    }

    public
    func createColumnUserUuid() -> Self.UpdatableColumnWithValue {
        return .userUuid(userUuid)
    }

    public
    func createColumnFirstName() -> Self.UpdatableColumnWithValue {
        return .firstName(firstName)
    }

    public
    func createColumnJsonStruct() -> Self.UpdatableColumnWithValue {
        return .jsonStruct(jsonStruct)
    }

    public
    func createColumnJsonStructOptional() -> Self.UpdatableColumnWithValue {
        return .jsonStructOptional(jsonStructOptional)
    }

    public
    func createColumnJsonStructArray() -> Self.UpdatableColumnWithValue {
        return .jsonStructArray(jsonStructArray)
    }

    public
    func createColumnJsonStructArrayOptional() -> Self.UpdatableColumnWithValue {
        return .jsonStructArrayOptional(jsonStructArrayOptional)
    }

    public
    func createColumnInteger() -> Self.UpdatableColumnWithValue {
        return .integer(integer)
    }

    public
    func createColumnBool() -> Self.UpdatableColumnWithValue {
        return .bool(bool)
    }

    public
    func createColumnSerializedInfo() -> Self.UpdatableColumnWithValue {
        return .serializedInfo(serializedInfoAutoConvert())
    }

    public
    func createColumnSerializedInfoNullable() -> Self.UpdatableColumnWithValue {
        return .serializedInfoNullable(serializedInfoNullableAutoConvert())
    }

    public func genUpsertDynamic(db: Database, columns: [UpdatableColumn]) throws {
        // Check for duplicates
        assert(Set(columns).count == columns.count)

        if columns.isEmpty {
            return
        }

        var upsertQuery = DbUser.insertUniqueQuery + "on conflict (userUuid) do update set "
        var processedAtLeastOneColumns = false

        for column in columns {
            switch column {
            case .userUuid:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "userUuid=excluded.userUuid"
            case .firstName:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "firstName=excluded.firstName"
            case .jsonStruct:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "jsonStruct=excluded.jsonStruct"
            case .jsonStructOptional:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "jsonStructOptional=excluded.jsonStructOptional"
            case .jsonStructArray:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "jsonStructArray=excluded.jsonStructArray"
            case .jsonStructArrayOptional:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "jsonStructArrayOptional=excluded.jsonStructArrayOptional"
            case .integer:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "integer=excluded.integer"
            case .bool:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "bool=excluded.bool"
            case .serializedInfo:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "serializedInfo=excluded.serializedInfo"
            case .serializedInfoNullable:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "serializedInfoNullable=excluded.serializedInfoNullable"
            }

            processedAtLeastOneColumns = true
        }

        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            try {
                let data = try Shared.jsonEncoder.encode(self.jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try self.jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            try {
                let data = try Shared.jsonEncoder.encode(self.jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            try {
                try self.jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try! serializedInfo.serializedData(),
            try! serializedInfoNullable?.serializedData()
        ]

        Logging.log(upsertQuery, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: upsertQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public mutating func genUpsertDynamicMutate(db: Database, columns: [UpdatableColumnWithValue]) throws {
        for column in columns {
            column.update(entity: &self)
        }

        try genUpsertDynamic(db: db, columns: columns.map { $0.toUpdatableColumn() })
    }

    public
    static func genUpdateUserUuidAllRows(db: Database, userUuid: UUID) throws {
        let arguments: StatementArguments = try [
            userUuid.uuidString
        ]

        Logging.log("update User set userUuid = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set userUuid = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateFirstNameAllRows(db: Database, firstName: String?) throws {
        let arguments: StatementArguments = try [
            firstName
        ]

        Logging.log("update User set firstName = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set firstName = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateJsonStructAllRows(db: Database, jsonStruct: JsonType) throws {
        let arguments: StatementArguments = try [
            try {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }()
        ]

        Logging.log("update User set jsonStruct = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set jsonStruct = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateJsonStructOptionalAllRows(db: Database, jsonStructOptional: JsonType?) throws {
        let arguments: StatementArguments = try [
            try {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }()
        ]

        Logging.log("update User set jsonStructOptional = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set jsonStructOptional = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateJsonStructArrayAllRows(db: Database, jsonStructArray: [JsonType]) throws {
        let arguments: StatementArguments = try [
            try {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }()
        ]

        Logging.log("update User set jsonStructArray = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set jsonStructArray = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateJsonStructArrayOptionalAllRows(db: Database, jsonStructArrayOptional: [JsonType]?) throws {
        let arguments: StatementArguments = try [
            try {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }()
        ]

        Logging.log("update User set jsonStructArrayOptional = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set jsonStructArrayOptional = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateIntegerAllRows(db: Database, integer: Int) throws {
        let arguments: StatementArguments = try [
            integer
        ]

        Logging.log("update User set integer = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set integer = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateBoolAllRows(db: Database, bool: Bool) throws {
        let arguments: StatementArguments = try [
            bool
        ]

        Logging.log("update User set bool = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set bool = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateSerializedInfoAllRows(db: Database, serializedInfo: SerializedInfo) throws {
        let arguments: StatementArguments = try [
            try! serializedInfo.serializedData()
        ]

        Logging.log("update User set serializedInfo = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set serializedInfo = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateSerializedInfoNullableAllRows(db: Database, serializedInfoNullable: SerializedInfo?) throws {
        let arguments: StatementArguments = try [
            try! serializedInfoNullable?.serializedData()
        ]

        Logging.log("update User set serializedInfoNullable = ?", statementArguments: arguments)

        let statement = try db.cachedStatement(sql: "update User set serializedInfoNullable = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genSelectAll(db: Database) throws -> [DbUser] {
        Logging.log(selectAllQuery, statementArguments: .init())

        let statement = try db.cachedStatement(sql: selectAllQuery)

        return try DbUser.fetchAll(statement)
    }

    public
    static func genSelectCount(db: Database) throws -> Int {
        Logging.log(selectCountQuery, statementArguments: .init())

        let statement = try db.cachedStatement(sql: selectCountQuery)

        return try Int.fetchOne(statement)!
    }

    // Write the primary key struct, useful for selecting or deleting a unique row
    public struct PrimaryKey {
        // Static queries
        public static let selectQuery = "select * from User where userUuid = ?"
        public static let selectExistsQuery = "select exists(select 1 from User where userUuid = ?)"
        public static let deleteQuery = "delete from User where userUuid = ?"

        // Mapped columns to properties
        public var userUuid: UUID

        // Default initializer
        public init(userUuid: UUID) {
            self.userUuid = userUuid
        }

        // Queries a unique row in the database, the row may or may not exist
        public func genSelect(db: Database) throws -> DbUser? {
            let arguments: StatementArguments = try [
                userUuid.uuidString
            ]

            Logging.log(Self.selectQuery, statementArguments: arguments)

            let statement = try db.cachedStatement(sql: Self.selectQuery)

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

        // Checks if a row exists
        public func genSelectExists(db: Database) throws -> Bool {
            let arguments: StatementArguments = try [
                userUuid.uuidString
            ]

            Logging.log(Self.selectExistsQuery, statementArguments: arguments)

            let statement = try db.cachedStatement(sql: Self.selectExistsQuery)

            statement.setUncheckedArguments(arguments)

            // This always returns a row
            return try Bool.fetchOne(statement)!
        }

        // Deletes a unique row, asserts that the row actually existed
        public func genDelete(db: Database, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: Self.deleteQuery)

            Logging.log(Self.deleteQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateUserUuid(db: Database, userUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                userUuid.uuidString,
                self.userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateUserUuidQuery)

            Logging.log(DbUser.UpdatableColumn.updateUserUuidQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateFirstName(db: Database, firstName: String?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                firstName,
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateFirstNameQuery)

            Logging.log(DbUser.UpdatableColumn.updateFirstNameQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateJsonStruct(db: Database, jsonStruct: JsonType, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try {
                    let data = try Shared.jsonEncoder.encode(jsonStruct)
                    return String(data: data, encoding: .utf8)!
                }(),
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateJsonStructQuery)

            Logging.log(DbUser.UpdatableColumn.updateJsonStructQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateJsonStructOptional(db: Database, jsonStructOptional: JsonType?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try {
                    try jsonStructOptional.map {
                        let data = try Shared.jsonEncoder.encode($0)
                        return String(data: data, encoding: .utf8)!
                    }
                }(),
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateJsonStructOptionalQuery)

            Logging.log(DbUser.UpdatableColumn.updateJsonStructOptionalQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateJsonStructArray(db: Database, jsonStructArray: [JsonType], assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try {
                    let data = try Shared.jsonEncoder.encode(jsonStructArray)
                    return String(data: data, encoding: .utf8)!
                }(),
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateJsonStructArrayQuery)

            Logging.log(DbUser.UpdatableColumn.updateJsonStructArrayQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateJsonStructArrayOptional(db: Database, jsonStructArrayOptional: [JsonType]?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try {
                    try jsonStructArrayOptional.map {
                        let data = try Shared.jsonEncoder.encode($0)
                        return String(data: data, encoding: .utf8)!
                    }
                }(),
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateJsonStructArrayOptionalQuery)

            Logging.log(DbUser.UpdatableColumn.updateJsonStructArrayOptionalQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateInteger(db: Database, integer: Int, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                integer,
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateIntegerQuery)

            Logging.log(DbUser.UpdatableColumn.updateIntegerQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateBool(db: Database, bool: Bool, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                bool,
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateBoolQuery)

            Logging.log(DbUser.UpdatableColumn.updateBoolQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateSerializedInfo(db: Database, serializedInfo: SerializedInfo, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try! serializedInfo.serializedData(),
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateSerializedInfoQuery)

            Logging.log(DbUser.UpdatableColumn.updateSerializedInfoQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateSerializedInfoNullable(db: Database, serializedInfoNullable: SerializedInfo?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                try! serializedInfoNullable?.serializedData(),
                userUuid.uuidString
            ]

            let statement = try db.cachedStatement(sql: DbUser.UpdatableColumn.updateSerializedInfoNullableQuery)

            Logging.log(DbUser.UpdatableColumn.updateSerializedInfoNullableQuery, statementArguments: arguments)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public
        func genUpdateDynamic(db: Database, columns: [DbUser.UpdatableColumnWithValue], assertOneRowAffected: Bool = true, assertAtLeastOneUpdate: Bool = true) throws {
            assert(!assertAtLeastOneUpdate || !columns.isEmpty)

            // Check for duplicates
            assert(Set(columns.map { $0.columnName }).count == columns.count)

            if columns.isEmpty {
                return
            }

            let pkQuery = "where userUuid = ?"
            var updateQuery = "update User set "
            var arguments = StatementArguments()

            for column in columns {
                switch column {
                case let .userUuid(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value.uuidString]

                    updateQuery += "userUuid = ?"
                case let .firstName(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value]

                    updateQuery += "firstName = ?"
                case let .jsonStruct(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [try {
                        let data = try Shared.jsonEncoder.encode(value)
                        return String(data: data, encoding: .utf8)!
                    }()]

                    updateQuery += "jsonStruct = ?"
                case let .jsonStructOptional(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [try {
                        try value.map {
                            let data = try Shared.jsonEncoder.encode($0)
                            return String(data: data, encoding: .utf8)!
                        }
                    }()]

                    updateQuery += "jsonStructOptional = ?"
                case let .jsonStructArray(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [try {
                        let data = try Shared.jsonEncoder.encode(value)
                        return String(data: data, encoding: .utf8)!
                    }()]

                    updateQuery += "jsonStructArray = ?"
                case let .jsonStructArrayOptional(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [try {
                        try value.map {
                            let data = try Shared.jsonEncoder.encode($0)
                            return String(data: data, encoding: .utf8)!
                        }
                    }()]

                    updateQuery += "jsonStructArrayOptional = ?"
                case let .integer(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value]

                    updateQuery += "integer = ?"
                case let .bool(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value]

                    updateQuery += "bool = ?"
                case let .serializedInfo(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [try! value.serializedData()]

                    updateQuery += "serializedInfo = ?"
                case let .serializedInfoNullable(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [try! value?.serializedData()]

                    updateQuery += "serializedInfoNullable = ?"
                }
            }

            arguments += [userUuid.uuidString]

            let finalQuery = updateQuery + " " + pkQuery

            Logging.log(finalQuery, statementArguments: arguments)

            let statement = try db.cachedStatement(sql: finalQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public
        func genUpdate(db: Database, column: UpdatableColumnWithValue, assertOneRowAffected: Bool = true) throws {
            switch column {
            case let .userUuid(val): try genUpdateUserUuid(db: db, userUuid: val, assertOneRowAffected: assertOneRowAffected)
            case let .firstName(val): try genUpdateFirstName(db: db, firstName: val, assertOneRowAffected: assertOneRowAffected)
            case let .jsonStruct(val): try genUpdateJsonStruct(db: db, jsonStruct: val, assertOneRowAffected: assertOneRowAffected)
            case let .jsonStructOptional(val): try genUpdateJsonStructOptional(db: db, jsonStructOptional: val, assertOneRowAffected: assertOneRowAffected)
            case let .jsonStructArray(val): try genUpdateJsonStructArray(db: db, jsonStructArray: val, assertOneRowAffected: assertOneRowAffected)
            case let .jsonStructArrayOptional(val): try genUpdateJsonStructArrayOptional(db: db, jsonStructArrayOptional: val, assertOneRowAffected: assertOneRowAffected)
            case let .integer(val): try genUpdateInteger(db: db, integer: val, assertOneRowAffected: assertOneRowAffected)
            case let .bool(val): try genUpdateBool(db: db, bool: val, assertOneRowAffected: assertOneRowAffected)
            case let .serializedInfo(val): try genUpdateSerializedInfo(db: db, serializedInfo: val, assertOneRowAffected: assertOneRowAffected)
            case let .serializedInfoNullable(val): try genUpdateSerializedInfoNullable(db: db, serializedInfoNullable: val, assertOneRowAffected: assertOneRowAffected)
            }
        }
    }
}

extension DbUser: Identifiable {
    public var id: UUID {
        userUuid
    }
}
