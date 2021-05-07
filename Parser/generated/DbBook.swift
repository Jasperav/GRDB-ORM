// // This file is generated, do not edit

import Foundation
import GRDB

// Mapped table to struct
public struct DbBook: FetchableRecord, PersistableRecord, Codable, Equatable {
    // Static queries
    public static let insertUniqueQuery = "insert into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?)"
    public static let replaceUniqueQuery = "replace into Book (bookUuid, userUuid, integerOptional, tsCreated) values (?, ?, ?, ?)"
    public static let deleteAllQuery = "delete from Book"
    public static let updateUniqueQuery = "update Book set userUuid = ?, integerOptional = ?, tsCreated = ? where bookUuid = ?"

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
    public func primaryKey() -> DbBookPrimaryKey {
        .init(bookUuid: bookUuid)
    }

    public func genInsert(db: Database) throws {
        let statement = try db.cachedUpdateStatement(sql: Self.insertUniqueQuery)

        let arguments: StatementArguments = try [
            bookUuid.uuidString,
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
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
            bookUuid.uuidString,
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
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
            userUuid?.uuidString,
            integerOptional,
            tsCreated,
            bookUuid.uuidString,
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
public struct DbBookPrimaryKey {
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

    public func genSelect<T: DatabaseReader>(dbReader: T) throws -> DbBook? {
        try dbReader.read { database in
            try genSelect(db: database)
        }
    }

    // Same as function 'genSelectUnique', but throws an error when no record has been found
    public func genSelectExpect(db: Database) throws -> DbBook {
        if let instance = try genSelect(db: db) {
            return instance
        } else {
            throw DatabaseError(message: "Didn't found a record for \(self)")
        }
    }

    public func genSelectExpect<T: DatabaseReader>(dbReader: T) throws -> DbBook {
        try dbReader.read { database in
            try genSelectExpect(db: database)
        }
    }

    // Deletes a unique row, asserts that the row actually existed
    public func genDelete(db: Database) throws {
        let arguments: StatementArguments = try [
            bookUuid.uuidString,
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

    public enum UpdatableColumn {
        case userUuid, integerOptional, tsCreated

        public static let updateUserUuidQuery = "update Book set userUuid = ? where bookUuid = ?"
        public static let updateIntegerOptionalQuery = "update Book set integerOptional = ? where bookUuid = ?"
        public static let updateTsCreatedQuery = "update Book set tsCreated = ? where bookUuid = ?"
    }

    public func genUpdateUserUuid(db: Database, userUuid: UUID?) throws {
        let arguments: StatementArguments = try [
            userUuid?.uuidString,
            bookUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateUserUuidQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        assert(db.changesCount == 1)
    }

    public func genUpdateUserUuid<T: DatabaseWriter>(dbWriter: T, userUuid: UUID?) throws {
        try dbWriter.write { database in
            try genUpdateUserUuid(db: database, userUuid: userUuid)
        }
    }

    public func genUpdateIntegerOptional(db: Database, integerOptional: Int?) throws {
        let arguments: StatementArguments = try [
            integerOptional,
            bookUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateIntegerOptionalQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        assert(db.changesCount == 1)
    }

    public func genUpdateIntegerOptional<T: DatabaseWriter>(dbWriter: T, integerOptional: Int?) throws {
        try dbWriter.write { database in
            try genUpdateIntegerOptional(db: database, integerOptional: integerOptional)
        }
    }

    public func genUpdateTsCreated(db: Database, tsCreated: Int64) throws {
        let arguments: StatementArguments = try [
            tsCreated,
            bookUuid.uuidString,
        ]

        let statement = try db.cachedUpdateStatement(sql: Self.UpdatableColumn.updateTsCreatedQuery)

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        assert(db.changesCount == 1)
    }

    public func genUpdateTsCreated<T: DatabaseWriter>(dbWriter: T, tsCreated: Int64) throws {
        try dbWriter.write { database in
            try genUpdateTsCreated(db: database, tsCreated: tsCreated)
        }
    }
}
