// // This file is generated, do not edit

import Foundation
import GRDB

import Combine
import GRDBQuery
public extension DbBook {
    struct BooksForUserWithSpecificUuidType: Equatable {
        public let gen0: DbBook
        public let gen1: Int
        public let gen2: [JsonType]?
        public let gen3: Int
        public init(row: Row) {
            gen0 = DbBook(row: row, startingIndex: 0)
            gen1 = row[4]
            gen2 = {
                if row.hasNull(atIndex: 5) {
                    return nil
                } else {
                    return try! Shared.jsonDecoder.decode([JsonType].self, from: row[5])
                }
            }()
            gen3 = row[6]
        }
    }

    static func booksForUserWithSpecificUuid(db: Database, userUuid: UUID) throws -> [BooksForUserWithSpecificUuidType] {
        var query = """
        select Book.*, User.integer, User.jsonStructArrayOptional, 1 from Book
                            join User on User.userUuid = Book.userUuid
                            where User.userUuid = ?
        """
        var arguments = StatementArguments()
        arguments += [userUuid.uuidString]
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [BooksForUserWithSpecificUuidType] = try Row.fetchAll(statement).map { row -> BooksForUserWithSpecificUuidType in
            BooksForUserWithSpecificUuidType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct BooksForUserWithSpecificUuidQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let userUuid: UUID
        public init(
            userUuid: UUID,
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.userUuid = userUuid
            self.scheduler = scheduler
        }

        public static let defaultValue: [BooksForUserWithSpecificUuidType] = []

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.userUuid == rhs.userUuid
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[BooksForUserWithSpecificUuidType], Error> {
            ValueObservation
                .tracking { db in
                    try booksForUserWithSpecificUuid(db: db, userUuid: userUuid)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbBook {
    struct BooksWithOptionalUserType: Equatable {
        public let gen0: DbBook
        public let gen1: DbUser?
        public let gen2: Bool?
        public init(row: Row) {
            gen0 = DbBook(row: row, startingIndex: 0)
            gen1 = {
                if row.hasNull(atIndex: 4) {
                    return nil
                } else {
                    return DbUser(row: row, startingIndex: 4)
                }
            }()
            gen2 = row[14]
        }
    }

    static func booksWithOptionalUser(db: Database) throws -> [BooksWithOptionalUserType] {
        var query = """
        select Book.*, User.*, Book.integerOptional
                            from Book
                            left join User on User.userUuid = Book.userUuid
        """
        Logging.log(query, statementArguments: .init())

        let statement = try db.cachedStatement(sql: query)
        let converted: [BooksWithOptionalUserType] = try Row.fetchAll(statement).map { row -> BooksWithOptionalUserType in
            BooksWithOptionalUserType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct BooksWithOptionalUserQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler

        public init(
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.scheduler = scheduler
        }

        public static let defaultValue: [BooksWithOptionalUserType] = []

        public static func == (_: Self, _: Self) -> Bool {
            // TODO: not sure if this is correct
            true
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[BooksWithOptionalUserType], Error> {
            ValueObservation
                .tracking { db in
                    try booksWithOptionalUser(db: db)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    struct FindByUsernameType: Equatable {
        public let gen0: DbUser
        public init(row: Row) {
            gen0 = DbUser(row: row, startingIndex: 0)
        }
    }

    static func findByUsername(db: Database, firstName: String) throws -> FindByUsernameType? {
        var query = """
        select * from User where firstName = ?
        """
        var arguments = StatementArguments()
        arguments += [firstName]
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [FindByUsernameType] = try Row.fetchAll(statement).map { row -> FindByUsernameType in
            FindByUsernameType(row: row)
        }

        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct FindByUsernameQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let firstName: String
        public init(
            firstName: String,
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.firstName = firstName
            self.scheduler = scheduler
        }

        public static let defaultValue: FindByUsernameType? = nil

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.firstName == rhs.firstName
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<FindByUsernameType?, Error> {
            ValueObservation
                .tracking { db in
                    try findByUsername(db: db, firstName: firstName)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    struct FindUserUuidByUsernameType: Equatable {
        public let gen0: UUID
        public init(row: Row) {
            gen0 = row[0]
        }
    }

    static func findUserUuidByUsername(db: Database, firstName: String) throws -> FindUserUuidByUsernameType? {
        var query = """
        select userUuid from User where firstName = ?
        """
        var arguments = StatementArguments()
        arguments += [firstName]
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [FindUserUuidByUsernameType] = try Row.fetchAll(statement).map { row -> FindUserUuidByUsernameType in
            FindUserUuidByUsernameType(row: row)
        }

        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct FindUserUuidByUsernameQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let firstName: String
        public init(
            firstName: String,
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.firstName = firstName
            self.scheduler = scheduler
        }

        public static let defaultValue: FindUserUuidByUsernameType? = nil

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.firstName == rhs.firstName
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<FindUserUuidByUsernameType?, Error> {
            ValueObservation
                .tracking { db in
                    try findUserUuidByUsername(db: db, firstName: firstName)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    struct AmountOfUsersType: Equatable {
        public let gen0: Int
        public init(row: Row) {
            gen0 = row[0]
        }
    }

    static func amountOfUsers(db: Database) throws -> AmountOfUsersType? {
        var query = """
        select count(*) from User
        """
        Logging.log(query, statementArguments: .init())

        let statement = try db.cachedStatement(sql: query)
        let converted: [AmountOfUsersType] = try Row.fetchAll(statement).map { row -> AmountOfUsersType in
            AmountOfUsersType(row: row)
        }

        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct AmountOfUsersQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler

        public init(
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.scheduler = scheduler
        }

        public static let defaultValue: AmountOfUsersType? = nil

        public static func == (_: Self, _: Self) -> Bool {
            // TODO: not sure if this is correct
            true
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<AmountOfUsersType?, Error> {
            ValueObservation
                .tracking { db in
                    try amountOfUsers(db: db)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
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
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        try statement.execute()
    }
}

public extension DbBook {
    struct HasAtLeastOneBookType: Equatable {
        public let gen0: Bool
        public init(row: Row) {
            gen0 = row[0]
        }
    }

    static func hasAtLeastOneBook(db: Database) throws -> HasAtLeastOneBookType? {
        var query = """
        select exists(select 1 from Book)
        """
        Logging.log(query, statementArguments: .init())

        let statement = try db.cachedStatement(sql: query)
        let converted: [HasAtLeastOneBookType] = try Row.fetchAll(statement).map { row -> HasAtLeastOneBookType in
            HasAtLeastOneBookType(row: row)
        }

        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct HasAtLeastOneBookQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler

        public init(
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.scheduler = scheduler
        }

        public static let defaultValue: HasAtLeastOneBookType? = nil

        public static func == (_: Self, _: Self) -> Bool {
            // TODO: not sure if this is correct
            true
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<HasAtLeastOneBookType?, Error> {
            ValueObservation
                .tracking { db in
                    try hasAtLeastOneBook(db: db)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    struct SerializeInfoSingleType: Equatable {
        public let gen0: SerializedInfo
        public let gen1: SerializedInfo?
        public init(row: Row) {
            gen0 = try! SerializedInfo(serializedData: row[0])
            gen1 = {
                if row.hasNull(atIndex: 1) {
                    return nil
                } else {
                    return try! SerializedInfo(serializedData: row[1])
                }
            }()
        }
    }

    static func serializeInfoSingle(db: Database) throws -> SerializeInfoSingleType? {
        var query = """
        select serializedInfo, serializedInfoNullable from user limit 1
        """
        Logging.log(query, statementArguments: .init())

        let statement = try db.cachedStatement(sql: query)
        let converted: [SerializeInfoSingleType] = try Row.fetchAll(statement).map { row -> SerializeInfoSingleType in
            SerializeInfoSingleType(row: row)
        }

        assert(converted.count <= 1, "Expected 1 or zero rows")
        return converted.first
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct SerializeInfoSingleQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler

        public init(
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.scheduler = scheduler
        }

        public static let defaultValue: SerializeInfoSingleType? = nil

        public static func == (_: Self, _: Self) -> Bool {
            // TODO: not sure if this is correct
            true
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<SerializeInfoSingleType?, Error> {
            ValueObservation
                .tracking { db in
                    try serializeInfoSingle(db: db)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    struct SerializeInfoArrayType: Equatable {
        public let gen0: SerializedInfo
        public let gen1: SerializedInfo?
        public init(row: Row) {
            gen0 = try! SerializedInfo(serializedData: row[0])
            gen1 = {
                if row.hasNull(atIndex: 1) {
                    return nil
                } else {
                    return try! SerializedInfo(serializedData: row[1])
                }
            }()
        }
    }

    static func serializeInfoArray(db: Database) throws -> [SerializeInfoArrayType] {
        var query = """
        select serializedInfo, serializedInfoNullable from user
        """
        Logging.log(query, statementArguments: .init())

        let statement = try db.cachedStatement(sql: query)
        let converted: [SerializeInfoArrayType] = try Row.fetchAll(statement).map { row -> SerializeInfoArrayType in
            SerializeInfoArrayType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct SerializeInfoArrayQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler

        public init(
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.scheduler = scheduler
        }

        public static let defaultValue: [SerializeInfoArrayType] = []

        public static func == (_: Self, _: Self) -> Bool {
            // TODO: not sure if this is correct
            true
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[SerializeInfoArrayType], Error> {
            ValueObservation
                .tracking { db in
                    try serializeInfoArray(db: db)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
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
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        try statement.execute()
    }
}

public extension DbUser {
    struct AllWithProvidedFirstNamesType: Equatable {
        public let gen0: DbUser
        public init(row: Row) {
            gen0 = DbUser(row: row, startingIndex: 0)
        }
    }

    static func allWithProvidedFirstNames(db: Database, firstName: [String]) throws -> [AllWithProvidedFirstNamesType] {
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

        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [AllWithProvidedFirstNamesType] = try Row.fetchAll(statement).map { row -> AllWithProvidedFirstNamesType in
            AllWithProvidedFirstNamesType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct AllWithProvidedFirstNamesQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let firstName: [String]
        public init(
            firstName: [String],
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.firstName = firstName
            self.scheduler = scheduler
        }

        public static let defaultValue: [AllWithProvidedFirstNamesType] = []

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.firstName == rhs.firstName
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[AllWithProvidedFirstNamesType], Error> {
            ValueObservation
                .tracking { db in
                    try allWithProvidedFirstNames(db: db, firstName: firstName)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbUser {
    struct ComplexType: Equatable {
        public let gen0: DbUser
        public init(row: Row) {
            gen0 = DbUser(row: row, startingIndex: 0)
        }
    }

    static func complex(db: Database, firstNames0: [String], jsonStructOptional: JsonType, integer: [Int], serializedInfoNullable: SerializedInfo) throws -> [ComplexType] {
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
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [ComplexType] = try Row.fetchAll(statement).map { row -> ComplexType in
            ComplexType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct ComplexQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let firstNames0: [String]
        public let jsonStructOptional: JsonType
        public let integer: [Int]
        public let serializedInfoNullable: SerializedInfo
        public init(
            firstNames0: [String],
            jsonStructOptional: JsonType,
            integer: [Int],
            serializedInfoNullable: SerializedInfo,
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.firstNames0 = firstNames0
            self.jsonStructOptional = jsonStructOptional
            self.integer = integer
            self.serializedInfoNullable = serializedInfoNullable
            self.scheduler = scheduler
        }

        public static let defaultValue: [ComplexType] = []

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.firstNames0 == rhs.firstNames0 && lhs.jsonStructOptional == rhs.jsonStructOptional && lhs.integer == rhs.integer && lhs.serializedInfoNullable == rhs.serializedInfoNullable
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[ComplexType], Error> {
            ValueObservation
                .tracking { db in
                    try complex(db: db, firstNames0: firstNames0, jsonStructOptional: jsonStructOptional, integer: integer, serializedInfoNullable: serializedInfoNullable)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbParent {
    struct RetrieveOptionalUserValuesType: Equatable {
        public let gen0: UUID
        public let gen1: UUID?
        public let gen2: [JsonType]?
        public let gen3: [JsonType]?
        public init(row: Row) {
            gen0 = row[0]
            gen1 = row[1]
            gen2 = {
                if row.hasNull(atIndex: 2) {
                    return nil
                } else {
                    return try! Shared.jsonDecoder.decode([JsonType].self, from: row[2])
                }
            }()
            gen3 = {
                if row.hasNull(atIndex: 3) {
                    return nil
                } else {
                    return try! Shared.jsonDecoder.decode([JsonType].self, from: row[3])
                }
            }()
        }
    }

    static func retrieveOptionalUserValues(db: Database, parentUuid: UUID) throws -> [RetrieveOptionalUserValuesType] {
        var query = """
        select parentUuid, U.userUuid, jsonStructArray, jsonStructArrayOptional from Parent left join User U on U.userUuid = Parent.userUuid where parentUuid = ?
        """
        var arguments = StatementArguments()
        arguments += [parentUuid.uuidString]
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [RetrieveOptionalUserValuesType] = try Row.fetchAll(statement).map { row -> RetrieveOptionalUserValuesType in
            RetrieveOptionalUserValuesType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct RetrieveOptionalUserValuesQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let parentUuid: UUID
        public init(
            parentUuid: UUID,
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.parentUuid = parentUuid
            self.scheduler = scheduler
        }

        public static let defaultValue: [RetrieveOptionalUserValuesType] = []

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.parentUuid == rhs.parentUuid
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[RetrieveOptionalUserValuesType], Error> {
            ValueObservation
                .tracking { db in
                    try retrieveOptionalUserValues(db: db, parentUuid: parentUuid)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbParent {
    struct RetrieveOptionalUserValuesMappedType: Equatable {
        public let gen0: UUID
        public let gen1: UUID?
        public let gen2: [JsonType]?
        public let gen3: [JsonType]?
        public init(row: Row) {
            gen0 = row[0]
            gen1 = row[1]
            gen2 = {
                if row.hasNull(atIndex: 2) {
                    return nil
                } else {
                    return try! Shared.jsonDecoder.decode([JsonType].self, from: row[2])
                }
            }()
            gen3 = {
                if row.hasNull(atIndex: 3) {
                    return nil
                } else {
                    return try! Shared.jsonDecoder.decode([JsonType].self, from: row[3])
                }
            }()
        }
    }

    static func retrieveOptionalUserValuesMapped(db: Database, parentUuid: UUID) throws -> [RetrieveOptionalUserValuesMappedType] {
        var query = """
        select parentUuid, U.userUuid, jsonStructArray, jsonStructArrayOptional from Parent left join User U on U.userUuid = Parent.userUuid where parentUuid = ? order by Parent.userUuid
        """
        var arguments = StatementArguments()
        arguments += [parentUuid.uuidString]
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [RetrieveOptionalUserValuesMappedType] = try Row.fetchAll(statement).map { row -> RetrieveOptionalUserValuesMappedType in
            RetrieveOptionalUserValuesMappedType(row: row)
        }

        return converted
    }

    static func retrieveOptionalUserValuesMappedMapped(db: Database, parentUuid: UUID) throws -> [RetrieveOptionalUserValuesType] {
        var query = """
        select parentUuid, U.userUuid, jsonStructArray, jsonStructArrayOptional from Parent left join User U on U.userUuid = Parent.userUuid where parentUuid = ? order by Parent.userUuid
        """
        var arguments = StatementArguments()
        arguments += [parentUuid.uuidString]
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [RetrieveOptionalUserValuesType] = try Row.fetchAll(statement).map { row -> RetrieveOptionalUserValuesType in
            RetrieveOptionalUserValuesType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct RetrieveOptionalUserValuesMappedQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let parentUuid: UUID
        public init(
            parentUuid: UUID,
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.parentUuid = parentUuid
            self.scheduler = scheduler
        }

        public static let defaultValue: [RetrieveOptionalUserValuesMappedType] = []

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.parentUuid == rhs.parentUuid
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[RetrieveOptionalUserValuesMappedType], Error> {
            ValueObservation
                .tracking { db in
                    try retrieveOptionalUserValuesMapped(db: db, parentUuid: parentUuid)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}

public extension DbParent {
    struct LimitedType: Equatable {
        public let gen0: DbParent
        public init(row: Row) {
            gen0 = DbParent(row: row, startingIndex: 0)
        }
    }

    static func limited(db: Database, limit: Int) throws -> [LimitedType] {
        var query = """
        select * from Parent limit ?
        """
        var arguments = StatementArguments()
        arguments += [limit]
        Logging.log(query, statementArguments: arguments)

        let statement = try db.cachedStatement(sql: query)
        statement.setUncheckedArguments(arguments)
        let converted: [LimitedType] = try Row.fetchAll(statement).map { row -> LimitedType in
            LimitedType(row: row)
        }

        return converted
    }

    // Very basic Queryable struct, create a PR if you want more customization
    struct LimitedQueryable: Queryable, Equatable {
        public let scheduler: ValueObservationScheduler
        public let limit: Int
        public init(
            limit: Int,
            scheduler: ValueObservationScheduler = .async(onQueue: .main)
        ) {
            self.limit = limit
            self.scheduler = scheduler
        }

        public static let defaultValue: [LimitedType] = []

        public static func == (lhs: Self, rhs: Self) -> Bool {
            lhs.limit == rhs.limit
        }

        public func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<[LimitedType], Error> {
            ValueObservation
                .tracking { db in
                    try limited(db: db, limit: limit)
                }
                .publisher(in: dbQueue, scheduling: scheduler)
                .eraseToAnyPublisher()
        }
    }
}
