import Foundation
import GRDB

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
                integer INTEGER NOT NULL
            );
        create table Book
            (
                bookUuid TEXT PRIMARY KEY NOT NULL,
                userUuid TEXT,
                integerOptional INTEGER,
                tsCreated INTEGER NOT NULL,
                FOREIGN KEY(userUuid) REFERENCES User(userUuid)
            );
        """)
    }

    return dbPool
}

extension DbUser {
    public static func random() -> DbUser {
        DbUser(userUuid: UUID(),
                firstName: nil,
                jsonStruct: JsonType(age: 1),
                jsonStructOptional: nil,
                jsonStructArray: [JsonType(age: 1)],
                jsonStructArrayOptional: nil,
                integer: 1,
                bool: true)
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
                bool: true)
    }
}
