use crate::line_writer::LineWriter;

pub fn write_data_extensions(line_writer: &mut LineWriter) {
    line_writer.add_line(
        "
    extension Data {
        public func serializedData() -> Data {
            self
        }
    }
    "
        .to_string(),
    )
}
