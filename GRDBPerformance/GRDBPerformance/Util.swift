import Foundation
import GRDB

public enum SerializedInfo: Equatable {
    case data(String)

    public init(serializedData: Data) {
        self = SerializedInfo.data(String(decoding: serializedData, as: UTF8.self))
    }

    public func serializedData() -> Data {
        switch self {
        case .data(let s): return s.data(using: .utf8)!
        }
    }
}

public func setupPool() -> DatabasePool {
    let url = try! FileManager.default
            .url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true)
            .appendingPathComponent("db.sqlite")

    // Remove the DB if it exists
    try? FileManager.default.removeItem(at: url)

    let dbPool = try! DatabasePool(path: url.path)

    try! dbPool.write { db in
        try! db.execute(sql:
        """
        create table User
            (
                userUuid TEXT PRIMARY KEY NOT NULL,
                firstName TEXT,
                jsonStruct TEXT NOT NULL,
                jsonStructOptional TEXT,
                jsonStructArray TEXT NOT NULL,
                jsonStructArrayOptional TEXT,
                integer INTEGER NOT NULL,
                bool INTEGER NOT NULL,
                serializedInfo BLOB NOT NULL,
                serializedInfoNullable BLOB
            );
        create table Book
            (
                bookUuid TEXT PRIMARY KEY NOT NULL,
                userUuid TEXT,
                integerOptional INTEGER,
                tsCreated INTEGER NOT NULL,
                FOREIGN KEY(userUuid) REFERENCES User(userUuid)
            );

        create table UserBook
            (
                bookUuid TEXT NOT NULL,
                userUuid TEXT NOT NULL,
                PRIMARY KEY (bookUuid, userUuid),
                FOREIGN KEY(bookUuid) REFERENCES Book(bookUuid),
                FOREIGN KEY(userUuid) REFERENCES User(userUuid)
            );
        """)
    }

    return dbPool
}

public let contentSerializedInfo = "Something"

extension DbUser {
    public static func random() -> DbUser {
        DbUser(userUuid: UUID(),
                firstName: "SomeName",
                jsonStruct: JsonType(age: 1),
                jsonStructOptional: nil,
                jsonStructArray: [JsonType(age: 1)],
                jsonStructArrayOptional: nil,
                integer: 1,
                bool: true,
                serializedInfo: SerializedInfo.data(contentSerializedInfo),
                serializedInfoNullable: nil)
    }
}

extension User {
    public static func random() -> User {
        User(userUuid: UUID(),
                firstName: nil,
                jsonStruct: JsonType(age: 1),
                jsonStructOptional: nil,
                jsonStructArray: [JsonType(age: 1)],
                jsonStructArrayOptional: nil,
                integer: 1,
                bool: true,
                serializedInfo: contentSerializedInfo.data(using: .utf8)!,
                serializedInfoNullable: nil)
    }
}
