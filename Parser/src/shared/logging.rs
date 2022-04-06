use crate::line_writer::LineWriter;

/// Writes the logging functionality
pub fn write_logging(line_writer: &mut LineWriter) {
    line_writer.add_comment("Will log in debug mode only");
    line_writer.add_line(
        "import OSLog

             struct Logging {
                #if DEBUG
                private static let logger = Logger(subsystem: \"GRDB-ORM\", category: \"Query logging\")
                #endif

                public static func log(_ query: String, statementArguments: StatementArguments) {
                    #if DEBUG
                    let maybeDatabaseValues = Mirror(reflecting: statementArguments.self).children.first { $0.label == \"values\" }?.value as? [DatabaseValue]
                    var surelyDatabaseValues = maybeDatabaseValues!
                    var queryChanged = query
                    var ranges: [Range<String.Index>] = []
                    var start = queryChanged.startIndex

                    while start < queryChanged.endIndex,
                          let range = queryChanged.range(of: \"?\", range: start ..< queryChanged.endIndex) {
                        ranges.append(range)
                        start = range.upperBound
                    }

                    for range in ranges.reversed() {
                        let arg = surelyDatabaseValues.removeFirst().description

                        queryChanged.replaceSubrange(range, with: arg)
                    }

                    logger.debug(\"Executing: \\(queryChanged)\")
                    #endif
                }
             }

    "
        .to_string(),
    );
}
