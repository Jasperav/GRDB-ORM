// // This file is generated, do not edit

import Foundation
import GRDB

public extension DbBook {
    typealias BooksForUserWithSpecificUuidType = [(DbBook, Int, [JsonType]?, Int)]

    static func booksForUserWithSpecificUuid(db: Database, userUuid: UUID) throws -> [(DbBook, Int, [JsonType]?, Int)] {
        let selectStatement = try db.cachedSelectStatement(sql: """
        select Book.*, User.integer, User.jsonStructArrayOptional, 1 from Book
                            join User on User.userUuid = Book.userUuid
                            where User.userUuid = ?
        """)
        selectStatement.setUncheckedArguments(StatementArguments(values: [userUuid.uuidString.databaseValue]))
        let converted: [(DbBook, Int, [JsonType]?, Int)] = try Row.fetchAll(selectStatement).map { row -> (DbBook, Int, [JsonType]?, Int) in
            (DbBook(row: row, startingIndex: 0), row[3], {
                if row.hasNull(atIndex: 4) {
                    return nil
                } else {
                    return try! Shared.jsonDecoder.decode([JsonType].self, from: row[4])
                }
            }(), row[5])
        }
        return converted
    }

    static func quickReadBooksForUserWithSpecificUuid<T: DatabaseReader>(db: T, userUuid: UUID) throws -> [(DbBook, Int, [JsonType]?, Int)] {
        try db.read { db in
            try Self.booksForUserWithSpecificUuid(db: db, userUuid: userUuid)
        }
    }
}

public extension DbUser {
    typealias FindByUsernameType = DbUser?

    static func findByUsername(db: Database, firstName: String) throws -> DbUser? {
        let selectStatement = try db.cachedSelectStatement(sql: """
        select * from User where firstName = ?
        """)
        selectStatement.setUncheckedArguments(StatementArguments(values: [firstName.databaseValue]))
        let converted: [DbUser] = try Row.fetchAll(selectStatement).map { row -> DbUser in
            DbUser(row: row, startingIndex: 0)
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    static func quickReadFindByUsername<T: DatabaseReader>(db: T, firstName: String) throws -> DbUser? {
        try db.read { db in
            try Self.findByUsername(db: db, firstName: firstName)
        }
    }
}

public extension DbUser {
    typealias FindUserUuidByUsernameType = UUID?

    static func findUserUuidByUsername(db: Database, firstName: String) throws -> UUID? {
        let selectStatement = try db.cachedSelectStatement(sql: """
        select userUuid from User where firstName = ?
        """)
        selectStatement.setUncheckedArguments(StatementArguments(values: [firstName.databaseValue]))
        let converted: [UUID] = try Row.fetchAll(selectStatement).map { row -> UUID in
            row[0]
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    static func quickReadFindUserUuidByUsername<T: DatabaseReader>(db: T, firstName: String) throws -> UUID? {
        try db.read { db in
            try Self.findUserUuidByUsername(db: db, firstName: firstName)
        }
    }
}

public extension DbUser {
    typealias AmountOfUsersType = Int

    static func amountOfUsers(db: Database) throws -> Int {
        let selectStatement = try db.cachedSelectStatement(sql: """
        select count(*) from User
        """)
        let converted: [Int] = try Row.fetchAll(selectStatement).map { row -> Int in
            row[0]
        }
        assert(converted.count == 1, "Expected 1 row")
        return converted.first!
    }

    static func quickReadAmountOfUsers<T: DatabaseReader>(db: T) throws -> Int {
        try db.read { db in
            try Self.amountOfUsers(db: db)
        }
    }
}
