use crate::line_writer::LineWriter;
use crate::swift_property::{
    SwiftProperty, SwiftPropertyDecoder, decode_swift_property, swift_properties_and_types,
};
use crate::table_meta_data::TableMetaData;

/// Writes 3 initializers:
///  - Default initializer
///  - Initializer with a row parameter with a starting index
///  - Initializer with a row parameter only
pub fn write_initializer(tmd: &mut TableMetaData) {
    write_default_initializer(tmd.line_writer, &tmd.swift_properties.iter().collect());
    write_row_initializer_with_starting_index(tmd);
    write_row_initializer_protocol(tmd);
}

/// Writes the default initializer (simple assigning)
#[allow(clippy::ptr_arg)] // Code doesn't compile with this lint
pub fn write_default_initializer(
    line_writer: &mut LineWriter,
    swift_properties: &Vec<&SwiftProperty>,
) {
    let arguments = swift_properties_and_types(swift_properties).join(", \n");
    let assign = swift_properties
        .iter()
        .map(|p| {
            if let Some((serialize, _)) = p.serialize_deserialize_blob(true) {
                serialize
            } else {
                format!("self.{a} = {a}", a = &p.swift_property_name)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    write_init_starting_index(line_writer, &arguments, &assign, "Default initializer", "");
}

/// Writes the row initializer with a starting index
/// This is needed for dynamic queries
fn write_row_initializer_with_starting_index(tmd: &mut TableMetaData) {
    let assign = tmd
        .swift_properties
        .iter()
        .enumerate()
        .map(|(index, swift_property)| {
            let decoder = RowInitializerSwiftPropertyDecoder { index };

            decode_swift_property(&decoder, swift_property, false)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write_init_starting_index(
        tmd.line_writer,
        "row: Row",
        &assign,
        "Row initializer",
        ", startingIndex: Int",
    )
}

/// Writes the row initializer without another parameter
/// This simply calls the row initializer with '0' as starting index
fn write_row_initializer_protocol(tmd: &mut TableMetaData) {
    tmd.line_writer
        .add_comment("The initializer defined by the protocol");
    tmd.line_writer.add_with_modifier(
        "init(row: Row) {
            self.init(row: row, startingIndex: 0)
        }
        "
        .to_string(),
    );
}

/// Shared writer for row initializer (with and without the starting index parameter)
fn write_init_starting_index(
    line_writer: &mut LineWriter,
    arguments: &str,
    assign: &str,
    comment: &'static str,
    parameter: &str,
) {
    line_writer.add_comment(comment);
    line_writer.add_with_modifier(format!(
        "init({}{}) {{
            {}
        }}
    ",
        arguments, parameter, assign
    ));
}

struct RowInitializerSwiftPropertyDecoder {
    index: usize,
}

/// This is the implementation for the row initializer for the Swift property decoding
impl SwiftPropertyDecoder for RowInitializerSwiftPropertyDecoder {
    /// Make sure to take the starting index into account
    fn row_index(&self) -> String {
        format!("{} + startingIndex", self.index)
    }

    /// Assigning is straightforward
    fn assign(&self, property_name: &str, decoded: &str) -> String {
        format!("{} = {}", property_name, decoded)
    }
}
