// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbBook: FetchableRecord, PersistableRecord, Codable, Equatable {
    // Static queries
    public static let insertUniqueQuery = "insert into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?)"
    public static let replaceUniqueQuery = "replace into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?)"
    public static let insertOrIgnoreUniqueQuery = "insert or ignore into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?)"
    public static let deleteAllQuery = "delete from Book"
    public static let updateUniqueQuery = "update Book set userUuid = ?, integerOptional = ?, tsCreated = ? where bookUuid = ?"
    public static let upsertUserUuidQuery = "insert into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?) on conflict (bookUuid) do update set userUuid=excluded.userUuid"
    public static let upsertIntegerOptionalQuery = "insert into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?) on conflict (bookUuid) do update set integerOptional=excluded.integerOptional"
    public static let upsertTsCreatedQuery = "insert into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?) on conflict (bookUuid) do update set tsCreated=excluded.tsCreated"

    // Mapped columns to properties
    public let bookUuid: UUID
    public var userUuid: UUID?
    public var integerOptional: Int?
    public var tsCreated: Int64

    // Default initializer
    public init(bookUuid: UUID,
                userUuid: UUID?,
                integerOptional: Int?,
                tsCreated: Int64)
    {
        self.bookUuid = bookUuid
        self.userUuid = userUuid
        self.integerOptional = integerOptional
        self.tsCreated = tsCreated
    }

    // Row initializer
    public init(row: Row, startingIndex: Int) {
        bookUuid = row[0 + startingIndex]
        userUuid = row[1 + startingIndex]
        integerOptional = row[2 + startingIndex]
        tsCreated = row[3 + startingIndex]
    }

    // The initializer defined by the protocol
    public init(row: Row) {
        self.init(row: row, startingIndex: 0)
    }

    // Easy way to get the PrimaryKey from the table
    public func primaryKey() -> PrimaryKey {
        .init(bookUuid: bookUuid)
    }

    public func genInsert(db: Database, assertOneRowAffected: Bool = true) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.insertUniqueQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
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
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genReplace(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.replaceUniqueQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genUpsertUserUuid(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.upsertUserUuidQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genUpsertIntegerOptional(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.upsertIntegerOptionalQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
        ]

        statement.setUncheckedArguments(arguments)

        try statement.execute()
    }

    public func genUpsertTsCreated(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.upsertTsCreatedQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
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
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
            bookUuid.uuidString,
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
        public static let selectQuery = "select * from Book where bookUuid = ?"
        public static let deleteQuery = "delete from Book where bookUuid = ?"

        // Mapped columns to properties
        public let bookUuid: UUID

        // Default initializer
        public init(bookUuid: UUID) {
            self.bookUuid = bookUuid
        }

        // Queries a unique row in the database, the row may or may not exist
        public func genSelect(db: Database) throws -> DbBook? {
            let arguments: StatementArguments = try [
                bookUuid.uuidString,
            ]

            let statement = try db.cachedSelectStatement(sql: Self.selectQuery)

            statement.setUncheckedArguments(arguments)

            return try DbBook.fetchOne(statement)
        }

        // Same as function 'genSelectUnique', but throws an error when no record has been found
        public func genSelectExpect(db: Database) throws -> DbBook {
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
            ]

            let statement = try db.cachedUpdateStatement(sql: Self.deleteQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public enum UpdatableColumn {
            case bookUuid, userUuid, integerOptional, tsCreated

            public static let updateBookUuidQuery = "update Book set bookUuid = ? where bookUuid = ?"
            public static let updateUserUuidQuery = "update Book set userUuid = ? where bookUuid = ?"
            public static let updateIntegerOptionalQuery = "update Book set integerOptional = ? where bookUuid = ?"
            public static let updateTsCreatedQuery = "update Book set tsCreated = ? where bookUuid = ?"
        }

        public func genUpdateBookUuid(db: Database, bookUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                bookUuid.uuidString,
                self.bookUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateBookUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateUserUuid(db: Database, userUuid: UUID?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                userUuid?.uuidString,
                bookUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateUserUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateIntegerOptional(db: Database, integerOptional: Int?, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                integerOptional,
                bookUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateIntegerOptionalQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateTsCreated(db: Database, tsCreated: Int64, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                tsCreated,
                bookUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: UpdatableColumn.updateTsCreatedQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }
    }
}
