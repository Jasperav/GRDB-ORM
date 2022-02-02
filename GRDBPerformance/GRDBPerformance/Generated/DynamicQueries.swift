// // This file is generated, do not edit

import Foundation
import GRDB

import Combine
public extension DbBook {
    typealias BooksForUserWithSpecificUuidType = [(DbBook, Int, [JsonType]?, Int)]

    static func booksForUserWithSpecificUuid(db: Database, userUuid: UUID) throws -> [(DbBook, Int, [JsonType]?, Int)] {
        var query = """
        select Book.*, User.integer, User.jsonStructArrayOptional, 1 from Book
                            join User on User.userUuid = Book.userUuid
                            where User.userUuid = ?
        """
        var arguments = StatementArguments()
        arguments += [userUuid.uuidString]
        let statement = try db.cachedStatement(sql: query)
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

    // Very basic Queryable struct, create a PR if you want more customization
    struct BooksForUserWithSpecificUuidQueryable: Queryable {
        public let userUuid: UUID
        public init(
            userUuid: UUID
        ) {
            self.userUuid = userUuid
        }

        static let defaultValue: BooksForUserWithSpecificUuidType = []

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<BooksForUserWithSpecificUuidType, Error> {
            ValueObservation
                .tracking { db in
                    try booksForUserWithSpecificUuid(db: db, userUuid: userUuid)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    typealias FindByUsernameType = DbUser?

    static func findByUsername(db: Database, firstName: String) throws -> DbUser? {
        var query = """
        select * from User where firstName = ?
        """
        var arguments = StatementArguments()
        arguments += [firstName]
        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [DbUser] = try Row.fetchAll(statement).map { row -> DbUser in
            DbUser(row: row, startingIndex: 0)
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct FindByUsernameQueryable: Queryable {
        public let firstName: String
        public init(
            firstName: String
        ) {
            self.firstName = firstName
        }

        static let defaultValue: FindByUsernameType = nil

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<FindByUsernameType, Error> {
            ValueObservation
                .tracking { db in
                    try findByUsername(db: db, firstName: firstName)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    typealias FindUserUuidByUsernameType = UUID?

    static func findUserUuidByUsername(db: Database, firstName: String) throws -> UUID? {
        var query = """
        select userUuid from User where firstName = ?
        """
        var arguments = StatementArguments()
        arguments += [firstName]
        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [UUID] = try Row.fetchAll(statement).map { row -> UUID in
            row[0]
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct FindUserUuidByUsernameQueryable: Queryable {
        public let firstName: String
        public init(
            firstName: String
        ) {
            self.firstName = firstName
        }

        static let defaultValue: FindUserUuidByUsernameType = nil

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<FindUserUuidByUsernameType, Error> {
            ValueObservation
                .tracking { db in
                    try findUserUuidByUsername(db: db, firstName: firstName)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    typealias AmountOfUsersType = Int?

    static func amountOfUsers(db: Database) throws -> Int? {
        var query = """
        select count(*) from User
        """
        let statement = try db.cachedStatement(sql: query)
        let converted: [Int] = try Row.fetchAll(statement).map { row -> Int in
            row[0]
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct AmountOfUsersQueryable: Queryable {
        static let defaultValue: AmountOfUsersType = nil

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<AmountOfUsersType, Error> {
            ValueObservation
                .tracking { db in
                    try amountOfUsers(db: db)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbBook {
    static func deleteByUserUuid(db: Database, userUuid: UUID) throws {
        var query = """
        delete from Book where userUuid = ?
        """
        var arguments = StatementArguments()
        arguments += [userUuid.uuidString]
        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        try statement.execute()
    }
}

public extension DbBook {
    typealias HasAtLeastOneBookType = Bool?

    static func hasAtLeastOneBook(db: Database) throws -> Bool? {
        var query = """
        select exists(select 1 from Book)
        """
        let statement = try db.cachedStatement(sql: query)
        let converted: [Bool] = try Row.fetchAll(statement).map { row -> Bool in
            row[0]
        }
        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct HasAtLeastOneBookQueryable: Queryable {
        static let defaultValue: HasAtLeastOneBookType = nil

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<HasAtLeastOneBookType, Error> {
            ValueObservation
                .tracking { db in
                    try hasAtLeastOneBook(db: db)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    typealias SerializeInfoSingleType = (SerializedInfo, SerializedInfo?)?

    static func serializeInfoSingle(db: Database) throws -> (SerializedInfo, SerializedInfo?)? {
        var query = """
        select serializedInfo, serializedInfoNullable from user limit 1
        """
        let statement = try db.cachedStatement(sql: query)
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

    // Very basic Queryable struct, create a PR if you want more customization
    struct SerializeInfoSingleQueryable: Queryable {
        static let defaultValue: SerializeInfoSingleType = nil

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<SerializeInfoSingleType, Error> {
            ValueObservation
                .tracking { db in
                    try serializeInfoSingle(db: db)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    typealias SerializeInfoArrayType = [(SerializedInfo, SerializedInfo?)]

    static func serializeInfoArray(db: Database) throws -> [(SerializedInfo, SerializedInfo?)] {
        var query = """
        select serializedInfo, serializedInfoNullable from user
        """
        let statement = try db.cachedStatement(sql: query)
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

    // Very basic Queryable struct, create a PR if you want more customization
    struct SerializeInfoArrayQueryable: Queryable {
        static let defaultValue: SerializeInfoArrayType = []

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<SerializeInfoArrayType, Error> {
            ValueObservation
                .tracking { db in
                    try serializeInfoArray(db: db)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    static func serializeInfoArray(db: Database, serializedInfo: SerializedInfo, serializedInfoNullable: SerializedInfo, firstName: String) throws {
        var query = """
        update user set serializedInfo = ? and serializedInfoNullable = ? where firstName = ?
        """
        var arguments = StatementArguments()
        arguments += [try! serializedInfo.serializedData()]
        arguments += [try! serializedInfoNullable.serializedData()]
        arguments += [firstName]
        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        try statement.execute()
    }
}

public extension DbUser {
    typealias AllWithProvidedFirstNamesType = [DbUser]

    static func allWithProvidedFirstNames(db: Database, firstName: [String]) throws -> [DbUser] {
        var query = """
        select * from user where firstName in %PARAM_IN%
        """
        var arguments = StatementArguments()
        if firstName.isEmpty {
            return []
        }

        for v in firstName {
            arguments += [v]
        }

        // Extra identifier is needed because else swift-format will format it incorrectly causing a compile error
        _ = {
            let questionMarks = String(repeating: "?, ", count: firstName.count)
            // Remove the trailing question mark
            let questionMarksCorrected = "(" + questionMarks.dropLast().dropLast() + ")"
            let occurrence = query.range(of: "%PARAM_IN%")!

            query = query.replacingCharacters(in: occurrence, with: questionMarksCorrected)
        }()

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [DbUser] = try Row.fetchAll(statement).map { row -> DbUser in
            DbUser(row: row, startingIndex: 0)
        }
        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct AllWithProvidedFirstNamesQueryable: Queryable {
        public let firstName: [String]
        public init(
            firstName: [String]
        ) {
            self.firstName = firstName
        }

        static let defaultValue: AllWithProvidedFirstNamesType = []

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<AllWithProvidedFirstNamesType, Error> {
            ValueObservation
                .tracking { db in
                    try allWithProvidedFirstNames(db: db, firstName: firstName)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    typealias ComplexType = [DbUser]

    static func complex(db: Database, firstNames0: [String], jsonStructOptional: JsonType, integer: [Int], serializedInfoNullable: SerializedInfo) throws -> [DbUser] {
        var query = """
        select * from user where firstName in %PARAM_IN% and jsonStructOptional = ? and integer in %PARAM_IN% and serializedInfoNullable = ?
        """
        var arguments = StatementArguments()
        if firstNames0.isEmpty {
            return []
        }

        for v in firstNames0 {
            arguments += [v]
        }

        // Extra identifier is needed because else swift-format will format it incorrectly causing a compile error
        _ = {
            let questionMarks = String(repeating: "?, ", count: firstNames0.count)
            // Remove the trailing question mark
            let questionMarksCorrected = "(" + questionMarks.dropLast().dropLast() + ")"
            let occurrence = query.range(of: "%PARAM_IN%")!

            query = query.replacingCharacters(in: occurrence, with: questionMarksCorrected)
        }()

        arguments += [try {
            let data = try Shared.jsonEncoder.encode(jsonStructOptional)
            return String(data: data, encoding: .utf8)!
        }()]
        if integer.isEmpty {
            return []
        }

        for v in integer {
            arguments += [v]
        }

        // Extra identifier is needed because else swift-format will format it incorrectly causing a compile error
        _ = {
            let questionMarks = String(repeating: "?, ", count: integer.count)
            // Remove the trailing question mark
            let questionMarksCorrected = "(" + questionMarks.dropLast().dropLast() + ")"
            let occurrence = query.range(of: "%PARAM_IN%")!

            query = query.replacingCharacters(in: occurrence, with: questionMarksCorrected)
        }()

        arguments += [try! serializedInfoNullable.serializedData()]
        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [DbUser] = try Row.fetchAll(statement).map { row -> DbUser in
            DbUser(row: row, startingIndex: 0)
        }
        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct ComplexQueryable: Queryable {
        public let firstNames0: [String]
        public let jsonStructOptional: JsonType
        public let integer: [Int]
        public let serializedInfoNullable: SerializedInfo
        public init(
            firstNames0: [String],
            jsonStructOptional: JsonType,
            integer: [Int],
            serializedInfoNullable: SerializedInfo
        ) {
            self.firstNames0 = firstNames0
            self.jsonStructOptional = jsonStructOptional
            self.integer = integer
            self.serializedInfoNullable = serializedInfoNullable
        }

        static let defaultValue: ComplexType = []

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<ComplexType, Error> {
            ValueObservation
                .tracking { db in
                    try complex(db: db, firstNames0: firstNames0, jsonStructOptional: jsonStructOptional, integer: integer, serializedInfoNullable: serializedInfoNullable)
                }
                .publisher(in: dbQueue)
                .eraseToAnyPublisher()
        }
    }
}
