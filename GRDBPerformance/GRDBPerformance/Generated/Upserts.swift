// // This file is generated, do not edit

import Foundation
import GRDB

public extension DbUser {
    func upsertExample(db: Database, assertOneRowAffected: Bool = true) throws {
        let arguments: StatementArguments = try [
            userUuid.uuidString,
            firstName,
            {
                let data = try Shared.jsonEncoder.encode(jsonStruct)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            {
                let data = try Shared.jsonEncoder.encode(jsonStructArray)
                return String(data: data, encoding: .utf8)!
            }(),
            {
                try jsonStructArrayOptional.map {
                    let data = try Shared.jsonEncoder.encode($0)
                    return String(data: data, encoding: .utf8)!
                }
            }(),
            integer,
            bool,
            try serializedInfo.serializedData(),
            try serializedInfoNullable?.serializedData(),
        ]

        let statement = try db.cachedUpdateStatement(sql: "insert into User (userUuid, firstName, jsonStruct, jsonStructOptional, jsonStructArray, jsonStructArrayOptional, integer, bool, serializedInfo, serializedInfoNullable) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) on conflict (userUuid) do update set jsonStruct=excluded.jsonStruct, jsonStructOptional=excluded.jsonStructOptional, integer=excluded.integer")

        statement.setUncheckedArguments(arguments)

        try statement.execute()

        if assertOneRowAffected {
            assert(db.changesCount == 1)
        }
    }

    func upsertExample<T: DatabaseWriter>(dbWriter: T) throws {
        try dbWriter.write { database in
            try upsertExample(db: database)
        }
    }
}
