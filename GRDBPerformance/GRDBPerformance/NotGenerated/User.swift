import Foundation
import GRDB

public struct User: FetchableRecord, PersistableRecord, Codable {
    
    public let userUuid: UUID
    public let firstName: String?
    public let jsonStruct: JsonType
    public let jsonStructOptional: JsonType?
    public let jsonStructArray: [JsonType]
    public let jsonStructArrayOptional: [JsonType]?
    public let integer: Int
    public let bool: Bool

    public static var databaseUUIDEncodingStrategy: DatabaseUUIDEncodingStrategy { .string }

    public init(userUuid: UUID, firstName: String?, jsonStruct: JsonType, jsonStructOptional: JsonType?, jsonStructArray: [JsonType], jsonStructArrayOptional: [JsonType]?, integer: Int, bool: Bool) {
        self.userUuid = userUuid
        self.firstName = firstName
        self.jsonStruct = jsonStruct
        self.jsonStructOptional = jsonStructOptional
        self.jsonStructArray = jsonStructArray
        self.jsonStructArrayOptional = jsonStructArrayOptional
        self.integer = integer
        self.bool = bool
    }
}
