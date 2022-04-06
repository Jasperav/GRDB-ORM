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

                public static func log(_ query: String, _ args: Any...) {
                    #if DEBUG
                    let argsToString = args.map { String(describing: $0) }.joined(separator: \", \")
                    let toAdd: String

                    if argsToString.isEmpty {
                        toAdd = \"\"
                    } else {
                        toAdd = \" with arguments: \" + argsToString
                    }

                    logger.debug(\"Executing: \\(query)\\(toAdd)\")
                    #endif
                }
             }

    "
        .to_string(),
    );
}
