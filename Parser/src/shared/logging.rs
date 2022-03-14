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

                public static func log(_ query: String) {
                    #if DEBUG
                    logger.debug(\"Executing: \\(query)\")
                    #endif
                }
             }

    "
        .to_string(),
    );
}
