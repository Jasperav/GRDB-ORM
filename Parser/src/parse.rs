use crate::configuration::Config;
use crate::dynamic_queries::parse::PARAMETERIZED_IN_QUERY;
use crate::line_writer::LineWriter;
use crate::swift_property::{create_swift_properties, encode_swift_properties, SwiftProperty};
use crate::swift_struct::TableWriter;
use rusqlite::{Error, NO_PARAMS};
use sqlite_parser::Metadata;
use std::ops::{Deref, DerefMut};

/// Starting point of parsing [Metadata] and [Config]
pub(crate) fn parse(tables: Metadata, config: Config) {
    // No tables? Something is wrong
    assert!(!tables.tables.is_empty());

    // Initialize the output dir
    let safe_output_dir = crate::output_dir_initializer::initialize(&config.output_dir);

    // Write the shared enum
    crate::shared::write(&config);

    // Write the tables
    TableWriter {
        tables: &tables,
        config: &config,
        safe_output_dir: safe_output_dir.clone(),
    }
    .write();

    let create_parser = || Parser::new(&config, &tables);

    // Write the dynamic queries
    create_parser().parse_dyn_queries();

    // Write the upserts
    create_parser().parse_upserts();

    // For the Swift code
    crate::format_swift_code::format_swift_code(&config, &safe_output_dir);
}

/// Parser for the dynamic queries and upserts
pub struct Parser<'a> {
    pub config: &'a Config,
    pub tables: &'a Metadata,
    pub line_writer: LineWriter,
}

impl<'a> Parser<'a> {
    pub fn new(config: &'a Config, tables: &'a Metadata) -> Parser<'a> {
        Self {
            config,
            tables,
            line_writer: config.create_line_writer(),
        }
    }

    pub fn write(self, file_name: &str) {
        self.line_writer.write_to_file(file_name);

        println!("Successfully processed");
    }

    pub fn find_swift_property(
        &self,
        table: &str,
        column: &str,
        param_name: &str,
        database_values: &mut Vec<String>,
    ) -> SwiftProperty {
        let table = self.tables.table(table).unwrap_or_else(|| {
            panic!("Did not found table {} in tables {:#?}", table, self.tables)
        });

        // Find the column in the table
        let mut swift_property = create_swift_properties(table, &self.config.custom_mapping)
            .iter()
            .find(|s| s.column.name.to_lowercase() == column.to_lowercase())
            .unwrap_or_else(|| {
                panic!(
                    "Couldn't find column '{}' in table '{}'",
                    column, table.table_name
                )
            })
            .clone();

        swift_property.make_not_null();

        // Rename the column to the parameter argument name, the param name gets precedence
        swift_property.swift_property_name = param_name.to_string();

        // Add the encoding functionality
        database_values.push(encode_swift_properties(&[&swift_property]));

        swift_property
    }
}

/// Tests a query
pub(crate) fn test_query(
    sqlite_location: &str,
    query: &str,
    return_types_is_empty: bool,
) -> String {
    // Start a connection to test the queries
    let connection = rusqlite::Connection::open(sqlite_location).unwrap();

    // Thanks to SQLite weak typing, all parameterized queries can be easily testing by executing it with '1'
    let query_for_validation = query
        .replace(" ?", " '1'")
        .replace(PARAMETERIZED_IN_QUERY, "(1)");

    println!("Validating query '{}'", query_for_validation);

    // Check if the query starts with select, delete or update. Insert and anything else are illegal
    // This is because insert queries are already generated expect if the insert also contains an ON CONFLICT clause
    let lowercased = query.to_lowercase();

    if !lowercased.starts_with("update ")
        && !lowercased.starts_with("select ")
        && !lowercased.starts_with("delete from ")
        && !lowercased.contains(" on conflict ")
    {
        panic!("Query should start with update, select, delete from or insert with on conflict");
    }

    if lowercased.starts_with("select ") {
        assert!(!return_types_is_empty)
    } else {
        assert!(return_types_is_empty)
    }

    let query = format!("explain {}", query_for_validation);

    if let Err(e) = connection.query_row(&query, NO_PARAMS, |_| Ok(())) {
        match e {
            Error::QueryReturnedNoRows => {
                // Fine
            }
            _ => panic!("Invalid query: {:#?}, error: {:#?}", query, e),
        }
    }

    query_for_validation
}

impl<'a> Deref for Parser<'a> {
    type Target = LineWriter;

    fn deref(&self) -> &Self::Target {
        &self.line_writer
    }
}

impl<'a> DerefMut for Parser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.line_writer
    }
}
