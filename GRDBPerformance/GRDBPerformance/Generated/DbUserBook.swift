// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbUserBook: FetchableRecord, PersistableRecord, Codable, Equatable {
    // Static queries
    public static let insertUniqueQuery = "insert into UserBook (bookUuid, userUuid) values (?, ?)"
    public static let replaceUniqueQuery = "replace into UserBook (bookUuid, userUuid) values (?, ?)"
    public static let deleteAllQuery = "delete from UserBook"

    // Mapped columns to properties
    public let bookUuid: UUID
    public let userUuid: UUID

    // Default initializer
    public init(bookUuid: UUID,
                userUuid: UUID)
    {
        self.bookUuid = bookUuid
        self.userUuid = userUuid
    }

    // Row initializer
    public init(row: Row, startingIndex: Int) {
        bookUuid = row[0 + startingIndex]
        userUuid = row[1 + startingIndex]
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

    // Write the primary key struct, useful for selecting or deleting a unique row
    public struct PrimaryKey {
        // Static queries
        public static let selectQuery = "select * from UserBook where bookUuid = ? and userUuid = ?"
        public static let deleteQuery = "delete from UserBook where bookUuid = ? and userUuid = ?"

        // Mapped columns to properties
        public let bookUuid: UUID
        public let userUuid: UUID

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

        public func genSelect<T: DatabaseReader>(dbReader: T) throws -> DbUserBook? {
            try dbReader.read { database in
                try genSelect(db: database)
            }
        }

        // Same as function 'genSelectUnique', but throws an error when no record has been found
        public func genSelectExpect(db: Database) throws -> DbUserBook {
            if let instance = try genSelect(db: db) {
                return instance
            } else {
                throw DatabaseError(message: "Didn't found a record for \(self)")
            }
        }

        public func genSelectExpect<T: DatabaseReader>(dbReader: T) throws -> DbUserBook {
            try dbReader.read { database in
                try genSelectExpect(db: database)
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

        public func genDelete<T: DatabaseWriter>(dbWriter: T) throws {
            try dbWriter.write { database in
                try genDelete(db: database)
            }
        }

        public enum UpdatableColumn {
            case bookUuid, userUuid

            public static let updateBookUuidQuery = "update UserBook set bookUuid = ? where bookUuid = ? and userUuid = ?"
            public static let updateUserUuidQuery = "update UserBook set userUuid = ? where bookUuid = ? and userUuid = ?"
        }

        public func genUpdateBookUuid(db: Database, bookUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                bookUuid.uuidString,
                self.bookUuid.uuidString,
                userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateBookUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateBookUuid<T: DatabaseWriter>(dbWriter: T, bookUuid: UUID) throws {
            try dbWriter.write { database in
                try genUpdateBookUuid(db: database, bookUuid: bookUuid)
            }
        }

        public func genUpdateUserUuid(db: Database, userUuid: UUID, assertOneRowAffected: Bool = true) throws {
            let arguments: StatementArguments = try [
                userUuid.uuidString,
                bookUuid.uuidString,
                self.userUuid.uuidString,
            ]

            let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateUserUuidQuery)

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            if assertOneRowAffected {
                assert(db.changesCount == 1)
            }
        }

        public func genUpdateUserUuid<T: DatabaseWriter>(dbWriter: T, userUuid: UUID) throws {
            try dbWriter.write { database in
                try genUpdateUserUuid(db: database, userUuid: userUuid)
            }
        }
    }
}
