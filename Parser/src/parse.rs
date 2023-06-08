use crate::android::AndroidWriter;
use crate::configuration::Config;
use crate::dynamic_queries::parse::{is_auto_generated_index, PARAMETERIZED_IN_QUERY};
use crate::line_writer::LineWriter;
use crate::swift_property::{create_swift_properties, encode_swift_properties, SwiftProperty};
use crate::swift_struct::TableWriter;
use grdb_orm_lib::dyn_query::DynamicQuery;
use regex::Regex;
use rusqlite::{Connection, Error};
use sqlite_parser::Metadata;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub struct Index {
    pub used: bool,
    pub amount_of_columns: i32,
}

/// Starting point of parsing [Metadata] and [Config]
pub(crate) fn parse(tables: Metadata, config: Config) {
    // No tables? Something is wrong
    assert!(!tables.tables.is_empty());

    parse_ios(&tables, &config);

    if config.output_dir_android.parent().is_none() {
        println!("Won't output android room objects because the output dir does not exists");

        return;
    }

    AndroidWriter {
        metadata: &tables,
        config: &config,
    }
    .parse();
}

fn parse_ios(tables: &Metadata, config: &Config) {
    // Initialize the output dir
    let safe_output_dir = crate::output_dir_initializer::initialize(&config.output_dir);

    // Write the shared enum
    crate::shared::write(config);

    // Write the metadata
    crate::metadata::write(config, tables);

    // Write the tables
    TableWriter {
        tables,
        config,
        safe_output_dir: safe_output_dir.clone(),
    }
    .write();

    // Write the dynamic queries
    Parser::new(config, tables).parse_dyn_queries();

    // For the Swift code
    crate::format_swift_code::format_swift_code(config, &safe_output_dir);
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
    dyn_query: &DynamicQuery,
    indexes: &mut HashMap<String, Index>,
) -> String {
    let query = &dyn_query.query;
    let return_types_is_empty = dyn_query.return_types.is_empty();
    // Thanks to SQLite weak typing, all parameterized queries can be easily testing by executing it with '1'
    let query_for_validation = query
        .replace(" ?", " '1'")
        .replace("(?", " ('1'")
        .replace(PARAMETERIZED_IN_QUERY, "(1)");

    println!(
        "Validating query for func name '{}', '{}'",
        dyn_query.func_name, query_for_validation
    );

    // Check if the query starts with select, delete or update. Insert and anything else are illegal
    // This is because insert queries are already generated expect if the insert also contains an ON CONFLICT clause
    let lowercased = query.to_lowercase();

    if lowercased.starts_with("select ") {
        assert!(!return_types_is_empty)
    } else {
        assert!(return_types_is_empty)
    }

    let query = format!("explain {}", query_for_validation);

    if let Err(e) = connection.query_row(&query, [], |_| Ok(())) {
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
        let mut prepared = connection.prepare(&query).unwrap();
        let mut rows = prepared.query([]).unwrap();
        let mut bypassed = false;

        println!("Preparing to find query plan: {}", query);

        let mut details = vec![];

        while let Some(row) = rows.next().unwrap() {
            let detail: String = row.get(3).unwrap();

            details.push(detail);
        }

        println!("Got {} rows, inner details: {:#?}", details.len(), details);

        for detail in details {
            let lowercased = detail.to_lowercase();
            let skippable = ["scalar subquery", "correlated scalar", "list subquery"];

            let mut should_continue = false;

            for skip in skippable {
                if lowercased.starts_with(skip) {
                    println!(
                        "Skip processing because it is a skippable detail: {}",
                        detail
                    );

                    should_continue = true;
                    break;
                }
            }

            if should_continue {
                continue;
            }

            if !lowercased.starts_with("search") {
                if dyn_query.bypass_index_optimizer {
                    println!("Bypassing query");

                    bypassed = true;
                } else {
                    panic!("Scanning tables is SLOW: {}", detail);
                }
            }

            let used_index = Regex::new(r"USING .*INDEX\s(\w+)").unwrap();

            if let Some(index) = used_index.captures(&detail) {
                let index = index.get(1).unwrap().as_str();

                println!("Found matching index: {}", index);

                if is_auto_generated_index(index) {
                    // Ignore
                    continue;
                }

                let sqlite_index = indexes.get_mut(index).expect(index);

                // Check if exactly all columns are used
                let question_marks = detail.matches('?').count();
                let amount_of_columns = sqlite_index.amount_of_columns as usize;
                let mut allowed_number_of_columns = vec![amount_of_columns];

                if query.contains("order by ") && amount_of_columns > 0 {
                    // For some reason, the order by is not included in the index
                    allowed_number_of_columns.push(amount_of_columns - 1);
                }

                if allowed_number_of_columns.contains(&question_marks) {
                    sqlite_index.used = true;
                }
            } else if !dyn_query.bypass_index_optimizer {
                panic!("No index was used");
            }
        }

        if !bypassed && dyn_query.bypass_index_optimizer {
            panic!("Did not bypass");
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
