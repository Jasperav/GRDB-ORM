// // This file is generated, do not edit

import Foundation
import GRDB

enum Shared {
    // JSONEncoder used for coding JSON columns
    static let jsonEncoder: JSONEncoder = {
        let encoder = JSONEncoder()

        encoder.dataEncodingStrategy = .base64
        encoder.dateEncodingStrategy = .millisecondsSince1970
        encoder.nonConformingFloatEncodingStrategy = .throw

        if #available(watchOS 4.0, OSX 10.13, iOS 11.0, tvOS 11.0, *) {
            // guarantee some stability in order to ease record comparison
            encoder.outputFormatting = .sortedKeys
        }

        return encoder
    }()

    // JSONDecoder used for coding JSON columns
    static let jsonDecoder: JSONDecoder = {
        let encoder = JSONDecoder()

        encoder.dataDecodingStrategy = .base64
        encoder.dateDecodingStrategy = .millisecondsSince1970
        encoder.nonConformingFloatDecodingStrategy = .throw

        return encoder
    }()
}

public extension Data {
    func serializedData() -> Data {
        self
    }
}

// Will log in debug mode only
import OSLog

struct Logging {
    #if DEBUG
        private static let logger = Logger(subsystem: "GRDB-ORM", category: "Query logging")
    #endif

    public static func log(_ query: String) {
        #if DEBUG
            logger.debug("Executing: \(query)")
        #endif
    }
}
