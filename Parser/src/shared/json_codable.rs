use crate::line_writer::LineWriter;

/// Writes the shared JSON encoding/decoding
pub fn write_json_coder(line_writer: &mut LineWriter) {
    line_writer.add_comment("JSONEncoder used for coding JSON columns");
    line_writer.add_line(
        "static let jsonEncoder: JSONEncoder = {
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
    "
        .to_string(),
    );

    line_writer.add_comment("JSONDecoder used for coding JSON columns");
    line_writer.add_line(
        "static let jsonDecoder: JSONDecoder = {
            let encoder = JSONDecoder()

            encoder.dataDecodingStrategy = .base64
            encoder.dateDecodingStrategy = .millisecondsSince1970
            encoder.nonConformingFloatDecodingStrategy = .throw

            return encoder
        }()
    "
        .to_string(),
    );
}
