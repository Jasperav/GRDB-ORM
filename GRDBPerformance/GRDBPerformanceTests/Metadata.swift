//
// Created by Jasper Visser on 07/08/2021.
//

import Foundation
import GRDBPerformance
import XCTest

class MetadataTest: XCTestCase {
    func testMetadata() throws {
        XCTAssertEqual(4, GenDbMetadata.tables().count)
    }
}
