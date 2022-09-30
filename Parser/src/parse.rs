use crate::configuration::Config;
use crate::dynamic_queries::parse::{is_auto_generated_index, PARAMETERIZED_IN_QUERY};
use crate::line_writer::LineWriter;
use crate::swift_property::{create_swift_properties, encode_swift_properties, SwiftProperty};
use crate::swift_struct::TableWriter;
use regex::Regex;
use rusqlite::{Connection, Error, NO_PARAMS};
use sqlite_parser::Metadata;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// Starting point of parsing [Metadata] and [Config]
pub(crate) fn parse(tables: Metadata, config: Config) {
    // No tables? Something is wrong
    assert!(!tables.tables.is_empty());

    // Initialize the output dir
    let safe_output_dir = crate::output_dir_initializer::initialize(&config.output_dir);

    // Write the shared enum
    crate::shared::write(&config);

    // Write the metadata
    crate::metadata::write(&config, &tables);

    // Write the tables
    TableWriter {
        tables: &tables,
        config: &config,
        safe_output_dir: safe_output_dir.clone(),
    }
    .write();

    // Write the dynamic queries
    Parser::new(&config, &tables).parse_dyn_queries();

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
    config: &Config,
    connection: &Connection,
    query: &str,
    return_types_is_empty: bool,
    indexes: &mut HashMap<String, bool>,
) -> String {
    // Thanks to SQLite weak typing, all parameterized queries can be easily testing by executing it with '1'
    let query_for_validation = query
        .replace(" ?", " '1'")
        .replace("(?", " ('1'")
        .replace(PARAMETERIZED_IN_QUERY, "(1)");

    println!("Validating query '{}'", query_for_validation);

    // Check if the query starts with select, delete or update. Insert and anything else are illegal
    // This is because insert queries are already generated expect if the insert also contains an ON CONFLICT clause
    let lowercased = query.to_lowercase();

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

    if config.index_optimizer {
        // Find used indexes
        let query = format!("explain query plan {}", query_for_validation);

        while let Some(row) = connection
            .prepare(&query)
            .unwrap()
            .query(NO_PARAMS)
            .unwrap()
            .next()
            .unwrap()
        {
            let detail: String = row.get(3).unwrap();
            let used_index = Regex::new(r"(?<=USING INDEX\s|USING INDEX\.\s)(\w+)").unwrap();

            if let Some(index) = used_index.find(&detail) {
                let index = index.as_str();

                if is_auto_generated_index(index) {
                    // Ignore
                    continue;
                }

                *indexes.get_mut(index).unwrap() = true;
            } else {
                panic!(
                    "No index was used, got other detail for query: {}, error:\n{:#?}",
                    query_for_validation, detail
                );
            }
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
