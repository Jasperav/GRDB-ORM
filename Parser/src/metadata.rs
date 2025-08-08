use crate::configuration::Config;
use crate::query_writer::main_struct::{DELETE_ALL_METHOD, SELECT_ALL_METHOD, SELECT_COUNT_METHOD};
use crate::swift_property::create_swift_type_name;
use sqlite_parser::Metadata;

pub const PROTOCOL_NAME: &str = "GenDbTable";
pub const PROTOCOL_WITH_SELF_NAME: &str = "GenDbTableWithSelf";

pub(crate) fn write(config: &Config, metadata: &Metadata) {
    write_protocol(config);
    write_metadata(config, metadata);
    write_protocol_with_self(config);
}

fn write_metadata(config: &Config, metadata: &Metadata) {
    let mut line_writer = config.create_line_writer();

    let mut tables = metadata.tables.values().cloned().collect::<Vec<_>>();

    tables.sort_by(|a, b| a.table_name.cmp(&b.table_name));

    let types = tables
        .into_iter()
        .map(|t| create_swift_type_name(&t.table_name, config))
        .map(|struct_name| format!("{struct_name}.self"))
        .collect::<Vec<_>>()
        .join(", ");

    line_writer.add_with_modifier(format!(
        "enum GenDbMetadata {{
        {}static func tables() -> [{}.Type] {{
            [{}]
        }}
    }}",
        config.visibility.modifier(),
        PROTOCOL_NAME,
        types
    ));

    line_writer.write_to_file("GenDbMetadata");
}

fn write_protocol_with_self(config: &Config) {
    let mut line_writer = config.create_line_writer();

    // Add more methods if needed
    line_writer.add_with_modifier(format!(
        "
    protocol {}: {} {{
        associatedtype {table}

        static func {}(db: Database) throws -> [{table}]
    }}
    ",
        PROTOCOL_WITH_SELF_NAME,
        PROTOCOL_NAME,
        SELECT_ALL_METHOD,
        table = "Table"
    ));

    line_writer.write_to_file(PROTOCOL_WITH_SELF_NAME);
}

fn write_protocol(config: &Config) {
    let mut line_writer = config.create_line_writer();

    // Add more methods if needed
    line_writer.add_with_modifier(format!(
        "
    protocol {PROTOCOL_NAME} {{
        static func {SELECT_COUNT_METHOD}(db: Database) throws -> Int
        static func {DELETE_ALL_METHOD}(db: Database) throws
    }}
    "
    ));

    line_writer.write_to_file(PROTOCOL_NAME);
}
