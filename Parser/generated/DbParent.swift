// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbParent: FetchableRecord, PersistableRecord, Codable, Equatable, Hashable, GenDbTable, GenDbTableWithSelf {
    // Static queries
    public static let insertUniqueQuery = "insert into Parent (parentUuid, userUuid) values (?, ?)"
    public static let replaceUniqueQuery = "replace into Parent (parentUuid, userUuid) values (?, ?)"
    public static let insertOrIgnoreUniqueQuery = "insert or ignore into Parent (parentUuid, userUuid) values (?, ?)"
    public static let deleteAllQuery = "delete from Parent"
    public static let selectAllQuery = "select parentUuid, userUuid from Parent"
    public static let selectCountQuery = "select count(*) from Parent"
    public static let updateUniqueQuery = "update Parent set userUuid = ? where parentUuid = ?"
    public static let upsertUserUuidQuery = "insert into Parent (parentUuid, userUuid) values (?, ?) on conflict (parentUuid) do update set userUuid=excluded.userUuid"

    // Mapped columns to properties
    public var parentUuid: UUID
    public var userUuid: UUID?

    // Default initializer
    public init(parentUuid: UUID,
                userUuid: UUID?) {
        self.parentUuid = parentUuid
        self.userUuid = userUuid
    }

    // Row initializer
    public init(row: Row, startingIndex: Int) {
        parentUuid = row[0 + startingIndex]
        userUuid = row[1 + startingIndex]
    }

    // The initializer defined by the protocol
    public init(row: Row) {
        self.init(row: row, startingIndex: 0)
    }

    // Easy way to get the PrimaryKey from the table
    public func primaryKey() -> PrimaryKey {
        .init(parentUuid: parentUuid)
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(parentUuid)
    }

    public func genInsert(db: Database, assertOneRowAffected: Bool = true) throws {
        Logging.log(Self.insertUniqueQuery, parentUuid, userUuid)

        let statement = try db.cachedStatement(sql: Self.insertUniqueQuery)

        let arguments: StatementArguments = try [
            parentUuid.uuidString,
            userUuid?.uuidString
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
    }

    public func genInsertOrIgnore(db: Database) throws {
        Logging.log(Self.insertOrIgnoreUniqueQuery, parentUuid, userUuid)

        let statement = try db.cachedStatement(sql: Self.insertOrIgnoreUniqueQuery)

        let arguments: StatementArguments = try [
            parentUuid.uuidString,
            userUuid?.uuidString
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genReplace(db: Database) throws {
        Logging.log(Self.replaceUniqueQuery, parentUuid, userUuid)

        let statement = try db.cachedStatement(sql: Self.replaceUniqueQuery)

        let arguments: StatementArguments = try [
            parentUuid.uuidString,
            userUuid?.uuidString
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genUpsertUserUuid(db: Database) throws {
        Logging.log(Self.upsertUserUuidQuery, userUuid)

        let statement = try db.cachedStatement(sql: Self.upsertUserUuidQuery)

        let arguments: StatementArguments = try [
            parentUuid.uuidString,
            userUuid?.uuidString
        ]

        statement.setUncheckedArguments(arguments)

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
        Logging.log(Self.deleteAllQuery)

        let statement = try db.cachedStatement(sql: Self.deleteAllQuery)

        try statement.execute()
    }

    public func genUpdate(db: Database, assertOneRowAffected: Bool = true) throws {
        Logging.log(Self.updateUniqueQuery, userUuid, parentUuid)

        let statement = try db.cachedStatement(sql: Self.updateUniqueQuery)

        let arguments: StatementArguments = try [
            userUuid?.uuidString,
            parentUuid.uuidString
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
    }

    public enum UpdatableColumn: String {
        case parentUuid, userUuid

        public static let updateParentUuidQuery = "update Parent set parentUuid = ? where parentUuid = ?"
        public static let updateUserUuidQuery = "update Parent set userUuid = ? where parentUuid = ?"
    }

    public enum UpdatableColumnWithValue {
        case parentUuid(UUID), userUuid(UUID?)

        var columnName: String {
            switch self {
            case .parentUuid: return "parentUuid"
            case .userUuid: return "userUuid"
            }
        }

        public func toUpdatableColumn() -> UpdatableColumn {
            switch self {
            case .parentUuid: return .parentUuid
            case .userUuid: return .userUuid
            }
        }

        public func update(entity: inout DbParent) {
            switch self {
            case let .parentUuid(value): entity.parentUuid = value
            case let .userUuid(value): entity.userUuid = value
            }
        }
    }

    public
    func createColumnParentUuid() -> Self.UpdatableColumnWithValue {
        return .parentUuid(parentUuid)
    }

    public
    func createColumnUserUuid() -> Self.UpdatableColumnWithValue {
        return .userUuid(userUuid)
    }

    public func genUpsertDynamic(db: Database, columns: [UpdatableColumn]) throws {
        // Check for duplicates
        assert(Set(columns).count == columns.count)

        if columns.isEmpty {
            return
        }

        var upsertQuery = DbParent.insertUniqueQuery + "on conflict (parentUuid) do update set "
        var processedAtLeastOneColumns = false

        for column in columns {
            switch column {
            case .parentUuid:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "parentUuid=excluded.parentUuid"
            case .userUuid:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "userUuid=excluded.userUuid"
            }

            processedAtLeastOneColumns = true
        }

        let arguments: StatementArguments = try [
            parentUuid.uuidString,
            userUuid?.uuidString
        ]

        Logging.log(upsertQuery, columns)

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
    static func genUpdateParentUuidAllRows(db: Database, parentUuid: UUID) throws {
        let arguments: StatementArguments = try [
            parentUuid.uuidString
        ]

        Logging.log("update Parent set parentUuid = ?", parentUuid)

        let statement = try db.cachedStatement(sql: "update Parent set parentUuid = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genUpdateUserUuidAllRows(db: Database, userUuid: UUID?) throws {
        let arguments: StatementArguments = try [
            userUuid?.uuidString
        ]

        Logging.log("update Parent set userUuid = ?", userUuid)

        let statement = try db.cachedStatement(sql: "update Parent set userUuid = ?")

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public
    static func genSelectAll(db: Database) throws -> [DbParent] {
        Logging.log(selectAllQuery)

        let statement = try db.cachedStatement(sql: selectAllQuery)

        return try DbParent.fetchAll(statement)
    }

    public
    static func genSelectCount(db: Database) throws -> Int {
        Logging.log(selectCountQuery)

        let statement = try db.cachedStatement(sql: selectCountQuery)

        return try Int.fetchOne(statement)!
    }

    // Write the primary key struct, useful for selecting or deleting a unique row
    public struct PrimaryKey {
        // Static queries
        public static let selectQuery = "select * from Parent where parentUuid = ?"
        public static let selectExistsQuery = "select exists(select 1 from Parent where parentUuid = ?)"
        public static let deleteQuery = "delete from Parent where parentUuid = ?"

        // Mapped columns to properties
        public var parentUuid: UUID

        // Default initializer
        public init(parentUuid: UUID) {
            self.parentUuid = parentUuid
        }

        // Queries a unique row in the database, the row may or may not exist
        public func genSelect(db: Database) throws -> DbParent? {
            let arguments: StatementArguments = try [
                parentUuid.uuidString
            ]

            Logging.log(Self.selectQuery)

            let statement = try db.cachedStatement(sql: Self.selectQuery)

            statement.setUncheckedArguments(arguments)

            return try DbParent.fetchOne(statement)
        }

        // Same as function 'genSelectUnique', but throws an error when no record has been found
        public func genSelectExpect(db: Database) throws -> DbParent {
            if let instance = try genSelect(db: db) {
                return instance
            } else {
                throw DatabaseError(message: "Didn't found a record for \(self)")
            }
        }

        // Checks if a row exists
        public func genSelectExists(db: Database) throws -> Bool {
            let arguments: StatementArguments = try [
                parentUuid.uuidString
            ]

            Logging.log(Self.selectExistsQuery)

            let statement = try db.cachedStatement(sql: Self.selectExistsQuery)

            statement.setUncheckedArguments(arguments)

            // This always returns a row
            return try Bool.fetchOne(statement)!
        }

        // Deletes a unique row, asserts that the row actually existed
        public func genDelete(db: Database, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                parentUuid.uuidString
            ]

            Logging.log(Self.deleteQuery)

            let statement = try db.cachedStatement(sql: Self.deleteQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateParentUuid(db: Database, parentUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                parentUuid.uuidString,
                self.parentUuid.uuidString
            ]

            Logging.log(DbParent.UpdatableColumn.updateParentUuidQuery, parentUuid)

            let statement = try db.cachedStatement(sql: DbParent.UpdatableColumn.updateParentUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateUserUuid(db: Database, userUuid: UUID?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                userUuid?.uuidString,
                parentUuid.uuidString
            ]

            Logging.log(DbParent.UpdatableColumn.updateUserUuidQuery, userUuid)

            let statement = try db.cachedStatement(sql: DbParent.UpdatableColumn.updateUserUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public
        func genUpdateDynamic(db: Database, columns: [DbParent.UpdatableColumnWithValue], assertOneRowAffected: Bool = true, assertAtLeastOneUpdate: Bool = true) throws {
            assert(!assertAtLeastOneUpdate || !columns.isEmpty)

            // Check for duplicates
            assert(Set(columns.map { $0.columnName }).count == columns.count)

            if columns.isEmpty {
                return
            }

            let pkQuery = "where parentUuid = ?"
            var updateQuery = "update Parent set "
            var arguments = StatementArguments()

            for column in columns {
                switch column {
                case let .parentUuid(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value.uuidString]

                    updateQuery += "parentUuid = ?"
                case let .userUuid(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value?.uuidString]

                    updateQuery += "userUuid = ?"
                }
            }

            arguments += [parentUuid.uuidString]

            let finalQuery = updateQuery + " " + pkQuery

            Logging.log(finalQuery, columns)

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
            case let .parentUuid(val): try genUpdateParentUuid(db: db, parentUuid: val, assertOneRowAffected: assertOneRowAffected)
            case let .userUuid(val): try genUpdateUserUuid(db: db, userUuid: val, assertOneRowAffected: assertOneRowAffected)
            }
        }
    }
}

extension DbParent: Identifiable {
    public var id: UUID {
        parentUuid
    }
}
