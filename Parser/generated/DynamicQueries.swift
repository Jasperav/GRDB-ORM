// // This file is generated, do not edit

import Foundation
import GRDB

public extension DbBook {
    typealias BooksForUserWithSpecificUuidType = [(DbBook, Int, [JsonType]?, Int)]

    static func booksForUserWithSpecificUuid(db: Database, userUuid: UUID) throws -> [(DbBook, Int, [JsonType]?, Int)] {
        let statement = try db.cachedSelectStatement(sql: """
        select Book.*, User.integer, User.jsonStructArrayOptional, 1 from Book
                            join User on User.userUuid = Book.userUuid
                            where User.userUuid = ?
        """)
        let arguments: StatementArguments = try [
            userUuid.uuidString,
        ]

        statement.setUncheckedArguments(arguments)
        let converted: [(DbBook, Int, [JsonType]?, Int)] = try Row.fetchAll(statement).map { row -> (DbBook, Int, [JsonType]?, Int) in
            (DbBook(row: row, startingIndex: 0), row[4], {
                if row.hasNull(atIndex: 5) {
                    return nil
                } else {
                    return try! Shared.jsonDecoder.decode([JsonType].self, from: row[5])
                }
            }(), row[6])
        }
        return converted
    }
}

public extension DbUser {
    typealias FindByUsernameType = DbUser?

    static func findByUsername(db: Database, firstName: String) throws -> DbUser? {
        let statement = try db.cachedSelectStatement(sql: """
        select * from User where firstName = ?
        """)
        let arguments: StatementArguments = try [
            firstName,
        ]

        statement.setUncheckedArguments(arguments)
        let converted: [DbUser] = try Row.fetchAll(statement).map { row -> DbUser in
            DbUser(row: row, startingIndex: 0)
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }
}

public extension DbUser {
    typealias FindUserUuidByUsernameType = UUID?

    static func findUserUuidByUsername(db: Database, firstName: String) throws -> UUID? {
        let statement = try db.cachedSelectStatement(sql: """
        select userUuid from User where firstName = ?
        """)
        let arguments: StatementArguments = try [
            firstName,
        ]

        statement.setUncheckedArguments(arguments)
        let converted: [UUID] = try Row.fetchAll(statement).map { row -> UUID in
            row[0]
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }
}

public extension DbUser {
    typealias AmountOfUsersType = Int?

    static func amountOfUsers(db: Database) throws -> Int? {
        let statement = try db.cachedSelectStatement(sql: """
        select count(*) from User
        """)
        let converted: [Int] = try Row.fetchAll(statement).map { row -> Int in
            row[0]
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }
}

public extension DbBook {
    static func deleteByUserUuid(db: Database, userUuid: UUID) throws {
        let statement = try db.cachedUpdateStatement(sql: """
        delete from Book where userUuid = ?
        """)
        let arguments: StatementArguments = try [
            userUuid.uuidString,
        ]

        statement.setUncheckedArguments(arguments)
        try statement.execute()
    }
}

public extension DbBook {
    typealias HasAtLeastOneBookType = Bool?

    static func hasAtLeastOneBook(db: Database) throws -> Bool? {
        let statement = try db.cachedSelectStatement(sql: """
        select exists(select 1 from Book)
        """)
        let converted: [Bool] = try Row.fetchAll(statement).map { row -> Bool in
            row[0]
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }
}

public extension DbUser {
    typealias SerializeInfoSingleType = (SerializedInfo, SerializedInfo?)?

    static func serializeInfoSingle(db: Database) throws -> (SerializedInfo, SerializedInfo?)? {
        let statement = try db.cachedSelectStatement(sql: """
        select serializedInfo, serializedInfoNullable from user limit 1
        """)
        let converted: [(SerializedInfo, SerializedInfo?)] = try Row.fetchAll(statement).map { row -> (SerializedInfo, SerializedInfo?) in
            (try! SerializedInfo(serializedData: row[0]), {
                if row.hasNull(atIndex: 1) {
                    return nil
                } else {
                    return try! SerializedInfo(serializedData: row[1])
                }
            }())
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }
}

public extension DbUser {
    typealias SerializeInfoArrayType = [(SerializedInfo, SerializedInfo?)]

    static func serializeInfoArray(db: Database) throws -> [(SerializedInfo, SerializedInfo?)] {
        let statement = try db.cachedSelectStatement(sql: """
        select serializedInfo, serializedInfoNullable from user
        """)
        let converted: [(SerializedInfo, SerializedInfo?)] = try Row.fetchAll(statement).map { row -> (SerializedInfo, SerializedInfo?) in
            (try! SerializedInfo(serializedData: row[0]), {
                if row.hasNull(atIndex: 1) {
                    return nil
                } else {
                    return try! SerializedInfo(serializedData: row[1])
                }
            }())
        }
        return converted
    }
}

public extension DbUser {
    static func serializeInfoArray(db: Database, serializedInfo: SerializedInfo, serializedInfoNullable: SerializedInfo, firstName: String) throws {
        let statement = try db.cachedUpdateStatement(sql: """
        update user set serializedInfo = ? and serializedInfoNullable = ? where firstName = ?
        """)
        let arguments: StatementArguments = try [
            try serializedInfo.serializedData(), try serializedInfoNullable.serializedData(), firstName,
        ]

        statement.setUncheckedArguments(arguments)
        try statement.execute()
    }
}
