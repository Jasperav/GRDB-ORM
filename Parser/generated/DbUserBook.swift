// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbUserBook: FetchableRecord, PersistableRecord, Codable, Equatable {
    // Static queries
    public static let insertUniqueQuery = "insert into UserBook (bookUuid, userUuid, realToDouble) values (?, ?, ?)"
    public static let replaceUniqueQuery = "replace into UserBook (bookUuid, userUuid, realToDouble) values (?, ?, ?)"
    public static let insertOrIgnoreUniqueQuery = "insert or ignore into UserBook (bookUuid, userUuid, realToDouble) values (?, ?, ?)"
    public static let deleteAllQuery = "delete from UserBook"
    public static let updateUniqueQuery = "update UserBook set realToDouble = ? where bookUuid = ? and userUuid = ?"
    public static let upsertRealToDoubleQuery = "insert into UserBook (bookUuid, userUuid, realToDouble) values (?, ?, ?) on conflict (bookUuid, userUuid) do update set realToDouble=excluded.realToDouble"

    // Mapped columns to properties
    public var bookUuid: UUID
    public var userUuid: UUID
    public var realToDouble: Double?

    // Default initializer
    public init(bookUuid: UUID,
                userUuid: UUID,
                realToDouble: Double?)
    {
        self.bookUuid = bookUuid
        self.userUuid = userUuid
        self.realToDouble = realToDouble
    }

    // Row initializer
    public init(row: Row, startingIndex: Int) {
        bookUuid = row[0 + startingIndex]
        userUuid = row[1 + startingIndex]
        realToDouble = row[2 + startingIndex]
    }

    // The initializer defined by the protocol
    public init(row: Row) {
        self.init(row: row, startingIndex: 0)
    }

    // Easy way to get the PrimaryKey from the table
    public func primaryKey() -> PrimaryKey {
        .init(bookUuid: bookUuid, userUuid: userUuid)
    }

    public func genInsert(db: Database, assertOneRowAffected: Bool = true) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.insertUniqueQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid.uuidString,
            realToDouble,
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
            bookUuid.uuidString,
            userUuid.uuidString,
            realToDouble,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genReplace(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.replaceUniqueQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid.uuidString,
            realToDouble,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genUpsertRealToDouble(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.upsertRealToDoubleQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid.uuidString,
            realToDouble,
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
            realToDouble,
            bookUuid.uuidString,
            userUuid.uuidString,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            // Only 1 row should be affected
            assert(db.changesCount == 1)
        }
    }

    public enum UpdatableColumn: String {
        case bookUuid, userUuid, realToDouble

        public static let updateBookUuidQuery = "update UserBook set bookUuid = ? where bookUuid = ? and userUuid = ?"
        public static let updateUserUuidQuery = "update UserBook set userUuid = ? where bookUuid = ? and userUuid = ?"
        public static let updateRealToDoubleQuery = "update UserBook set realToDouble = ? where bookUuid = ? and userUuid = ?"
    }

    public enum UpdatableColumnWithValue {
        case bookUuid(UUID), userUuid(UUID), realToDouble(Double?)

        var columnName: String {
            switch self {
            case .bookUuid: return "bookUuid"
            case .userUuid: return "userUuid"
            case .realToDouble: return "realToDouble"
            }
        }
    }

    public
    func createColumnBookUuid() -> Self.UpdatableColumnWithValue {
        return .bookUuid(bookUuid)
    }

    public
    func createColumnUserUuid() -> Self.UpdatableColumnWithValue {
        return .userUuid(userUuid)
    }

    public
    func createColumnRealToDouble() -> Self.UpdatableColumnWithValue {
        return .realToDouble(realToDouble)
    }

    public func genUpsertDynamic(db: Database, columns: [UpdatableColumn], assertAtLeastOneUpdate: Bool = true) throws {
        assert(!assertAtLeastOneUpdate || !columns.isEmpty)

        // Check for duplicates
        assert(Set(columns).count == columns.count)

        if columns.isEmpty {
            return
        }

        var upsertQuery = DbUserBook.insertUniqueQuery + "on conflict (bookUuid, userUuid) do update set "
        var processedAtLeastOneColumns = false

        for column in columns {
            switch column {
            case .bookUuid:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "bookUuid=excluded.bookUuid"
            case .userUuid:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "userUuid=excluded.userUuid"
            case .realToDouble:
                if processedAtLeastOneColumns {
                    upsertQuery += ", "
                }
                upsertQuery += "realToDouble=excluded.realToDouble"
            }

            processedAtLeastOneColumns = true
        }

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid.uuidString,
            realToDouble,
        ]

        let statement = try db.cachedUpdateStatement(sql: upsertQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    // Write the primary key struct, useful for selecting or deleting a unique row
    public struct PrimaryKey {
        // Static queries
        public static let selectQuery = "select * from UserBook where bookUuid = ? and userUuid = ?"
        public static let deleteQuery = "delete from UserBook where bookUuid = ? and userUuid = ?"

        // Mapped columns to properties
        public var bookUuid: UUID
        public var userUuid: UUID

        // Default initializer
        public init(bookUuid: UUID,
                    userUuid: UUID)
        {
            self.bookUuid = bookUuid
            self.userUuid = userUuid
        }

        // Queries a unique row in the database, the row may or may not exist
        public func genSelect(db: Database) throws -> DbUserBook? {
            let arguments: StatementArguments = try [
                bookUuid.uuidString,
                userUuid.uuidString,
            ]

            let statement = try db.cachedSelectStatement(sql: Self.selectQuery)

            statement.setUncheckedArguments(arguments)

            return try DbUserBook.fetchOne(statement)
        }

        // Same as function 'genSelectUnique', but throws an error when no record has been found
        public func genSelectExpect(db: Database) throws -> DbUserBook {
            if let instance = try genSelect(db: db) {
                return instance
            } else {
                throw DatabaseError(message: "Didn't found a record for \(self)")
            }
        }

        // Deletes a unique row, asserts that the row actually existed
        public func genDelete(db: Database, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                bookUuid.uuidString,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: Self.deleteQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateBookUuid(db: Database, bookUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                bookUuid.uuidString,
                self.bookUuid.uuidString,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: DbUserBook.UpdatableColumn.updateBookUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateUserUuid(db: Database, userUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                userUuid.uuidString,
                bookUuid.uuidString,
                self.userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: DbUserBook.UpdatableColumn.updateUserUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateRealToDouble(db: Database, realToDouble: Double?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                realToDouble,
                bookUuid.uuidString,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: DbUserBook.UpdatableColumn.updateRealToDoubleQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public
        func genUpdateDynamic(db: Database, columns: [DbUserBook.UpdatableColumnWithValue], assertOneRowAffected: Bool = true, assertAtLeastOneUpdate: Bool = true) throws {
            assert(!assertAtLeastOneUpdate || !columns.isEmpty)

            // Check for duplicates
            assert(Set(columns.map { $0.columnName }).count == columns.count)

            if columns.isEmpty {
                return
            }

            let pkQuery = "where bookUuid = ? and userUuid = ?"
            var updateQuery = "update DbUserBook "
            var arguments = StatementArguments()

            for column in columns {
                switch column {
                case let .bookUuid(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value.uuidString]

                    updateQuery += "set bookUuid = ?"
                case let .userUuid(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value.uuidString]

                    updateQuery += "set userUuid = ?"
                case let .realToDouble(value):
                    if !arguments.isEmpty {
                        updateQuery += ", "
                    }

                    arguments += [value]

                    updateQuery += "set realToDouble = ?"
                }
            }

            arguments += [bookUuid.uuidString]
            arguments += [userUuid.uuidString]

            let finalQuery = updateQuery + pkQuery

            let statement = try db.cachedUpdateStatement(sql: finalQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }
    }
}
