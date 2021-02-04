use std::ops::{Deref, DerefMut};

use rusqlite::{Connection, Error, NO_PARAMS};
use sqlite_parser::Metadata;

use crate::configuration::Config;
use crate::dynamic_queries::reader::DynamicQuery;
use crate::dynamic_queries::return_type::{ReturnType, ReturnTypeParsed};
use crate::line_writer::LineWriter;
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{
    create_swift_properties, create_swift_type_name, swift_properties_to_sqlite_database_values,
};

/// Parses a dynamic query
pub struct DynQueryParser<'a> {
    config: &'a Config,
    tables: &'a Metadata,
    line_writer: LineWriter,
}

impl<'a> DynQueryParser<'a> {
    pub fn new(config: &'a Config, tables: &'a Metadata) -> DynQueryParser<'a> {
        Self {
            config,
            tables,
            line_writer: config.create_line_writer(),
        }
    }

    /// Parses the dynamic queries
    pub fn parse(mut self) {
        if self.config.dynamic_queries.is_empty() {
            println!("No dynamic queries found");

            return;
        }

        println!("Preparing to process dynamic queries");

        self.add_line("import Foundation".to_string());

        // Start a connection to test the queries
        let connection = rusqlite::Connection::open(&self.config.sqlite_location).unwrap();

        self.add_line("import GRDB".to_string());
        self.new_line();

        for dynamic_query in &self.config.dynamic_queries {
            // Check if the query is valid
            test_query(&connection, &dynamic_query.query);

            // The parameters to invoke the Swift functions for
            let mut parameters = vec![];

            // The conversion of parameter to databaseValue
            let mut database_values = vec![];

            for (table, column, param_name) in &dynamic_query.parameter_types {
                // If the table equals FIXED, process it differently
                if table == "FIXED" {
                    // The last argument is the name of the parameter, while the second parameter
                    // is the Swift type of the parameter
                    parameters.push((param_name.to_string(), column.to_string()));

                    // The assumption is that '.databaseValue' can be called on the parameter
                    database_values.push(format!("{}.databaseValue", param_name));
                } else {
                    // If the table does not equal FIXED, it must be a table property
                    let table = self.tables.table(table).unwrap();

                    // Find the column in the table
                    let mut swift_property =
                        create_swift_properties(table, &self.config.custom_mapping)
                            .iter()
                            .find(|s| s.column.name.to_lowercase() == column.to_lowercase())
                            .unwrap_or_else(|| {
                                panic!(
                                    "Couldn't find column '{}' in table '{}'",
                                    column, table.table_name
                                )
                            })
                            .clone();

                    // Never should the argument be null (= null in DB doesn't make sense and is a bug)
                    // Replace the optional type with a nonnull type, regardless if the column is nullable
                    swift_property.swift_type.type_name =
                        swift_property.swift_type.type_name.replace('?', "");

                    // Rename the column to the parameter argument name, the param name gets precedence
                    swift_property.swift_property_name = param_name.clone();

                    // Add the decoding functionality
                    database_values.push(swift_properties_to_sqlite_database_values(vec![
                        &swift_property,
                    ]));

                    // Lastly, add the parameters
                    parameters.push((
                        param_name.to_string(),
                        swift_property.swift_type.type_name.clone(),
                    ));
                }
            }

            // Find out the return type
            let parsed = ReturnType {
                return_types: &dynamic_query.return_types,
                return_type_is_array: dynamic_query.return_types_is_array,
                tables: self.tables,
                config: self.config,
            }
            .parse();

            self.write_extension(&dynamic_query);

            let capitalized_func_name = self.write_type_alias(&dynamic_query, &parsed);
            let mut parameters_main = parameters.clone();

            // The db is always the first parameter
            parameters_main.insert(0, ("db".to_string(), "Database".to_string()));

            self.new_line();

            // Write the method declaration
            self.add_line(format!(
                "static func {}({}) throws -> {} {{",
                dynamic_query.func_name,
                separate_by_colon(&parameters_main),
                parsed.return_type
            ));

            // Replace the optional type here, no need for it.
            // This is needed for the type to map, this is always nonnull.
            // Only do this when there is a single type and it isn't an array, else e.g. (DbUser, SomeType?) will be corrupted
            let return_type = if !dynamic_query.return_types_is_array
                && dynamic_query.return_types[0].contains('?')
            {
                parsed.return_type.replace("?", "")
            } else {
                parsed.return_type.clone()
            };

            self.write_body(
                &dynamic_query,
                database_values,
                &return_type,
                &parsed.decoding,
            );

            // Add a convenience read method
            self.write_easy_read_method(
                &dynamic_query.func_name,
                &capitalized_func_name,
                &parsed.return_type,
                &parameters,
            );
            self.add_closing_brackets();
        }

        self.line_writer.write_to_file("DynamicQueries");

        println!("Successfully processed all dynamic queries");
    }

    /// Writes a Swift type alias for the return type, which makes it easy for the user to reuse the type
    fn write_type_alias(
        &mut self,
        dynamic_query: &&DynamicQuery,
        parsed: &ReturnTypeParsed,
    ) -> String {
        let capitalized_func_name = some_kind_of_uppercase_first_letter(&dynamic_query.func_name);
        let type_alias_name = format!("{}Type", capitalized_func_name);

        self.add_line(format!(
            "typealias {} = {}",
            type_alias_name, parsed.return_type
        ));

        capitalized_func_name
    }

    /// Writes the extension to add the methods to
    fn write_extension(&mut self, dynamic_query: &DynamicQuery) {
        // Find out the extension, if it matches a table, use that. Else use the raw extension input.
        let extension = if self.tables.table(&dynamic_query.extension).is_some() {
            create_swift_type_name(&dynamic_query.extension, self.config)
        } else {
            dynamic_query.extension.to_string()
        };

        self.add_with_modifier(format!("extension {} {{", extension));
    }

    /// Writes the 'easy' read method, the user doesn't have to write 'db.read {...}' for simple reads
    #[allow(clippy::ptr_arg)] // Code doesn't compile with this lint
    fn write_easy_read_method(
        &mut self,
        func_name: &str,
        capitalized_func_name: &str,
        return_type: &str,
        arguments: &Vec<(String, String)>,
    ) {
        // There is no customization on the name, it always starts with quickRead for now
        let quick_read_func_name = format!("quickRead{}", capitalized_func_name);
        let mut arguments_with_db = arguments.clone();

        // Add a generic parameter constraint on DatabaseReader, to maximize support
        arguments_with_db.insert(0, ("db".to_string(), "T".to_string()));

        let mut arguments_invocation = arguments
            .iter()
            .map(|(a, _)| format!("{b}: {b}", b = a))
            .collect::<Vec<_>>()
            .join(", ");

        if !arguments_invocation.is_empty() {
            // Add arguments if needed
            arguments_invocation = format!(", {}", arguments_invocation);
        }

        self.add_with_modifier(format!(
            "static func {}<T: DatabaseReader>({}) throws -> {} {{",
            quick_read_func_name,
            separate_by_colon(&arguments_with_db),
            return_type
        ));
        self.add_line(format!(
            "try db.read {{ db in
            try Self.{}(db: db{})
        }}",
            func_name, arguments_invocation
        ));
        self.add_closing_brackets();
    }

    /// Writes the actual body of the query processing
    fn write_body(
        &mut self,
        dynamic_query: &DynamicQuery,
        database_values: Vec<String>,
        return_value: &str,
        decoding: &str,
    ) {
        // Add the query as multiline text
        self.add_line(format!(
            "let selectStatement = try db.cachedSelectStatement(sql: \"\"\"\n{}\n\"\"\")",
            dynamic_query.query
        ));

        if !database_values.is_empty() {
            // Set unchecked arguments to the statement if there are arguments
            let separated = database_values.join(", ");

            self.add_line(format!(
                "selectStatement.setUncheckedArguments(StatementArguments(values: [{}]))",
                separated
            ));
        }

        // Remove the trailing and leading '[' and ']' and put that in return_value_closure
        // This is because in the closure, rows are processed one by one, and there is no need
        // that the return type is an array
        // Oddly enough, add brackets and put that in type_fetch_all, because when the return type
        // isn't an array, it will be validated though (makes no sense to 'just return the first one'
        // regardless of the resultset. If the user wants it, fine, add 'limit 1' to the query.
        let (return_value_closure, type_fetch_all) = if return_value.starts_with('[') {
            (
                &return_value[1..return_value.len() - 1],
                return_value.to_string(),
            )
        } else {
            (return_value, format!("[{}]", return_value))
        };

        // Add the converted property
        self.add_line(format!(
            "let converted: {} = try Row.fetchAll(selectStatement).map({{ row -> {} in",
            type_fetch_all, return_value_closure
        ));
        self.add_line(decoding.to_string());
        self.add_line("})".to_string());

        if dynamic_query.return_types_is_array {
            // if the return type is an array, it can be returned directly, no need for checking the resultset
            self.add_line("return converted".to_string());
        } else {
            // it isn't an array and the return type len is 1, check if it's nullable
            let suffix = if dynamic_query.return_types[0].contains('?') {
                // It nullable, the result set len should be 0 or 1
                self.add_line(
                    "assert(converted.count <= 1, \"Expected 1 or zero rows\")".to_string(),
                );

                ""
            } else {
                // If it's not nullable, than the result set len must be exactly 1
                self.add_line("assert(converted.count == 1, \"Expected 1 row\")".to_string());

                // Not optional return type, force unwrap
                "!"
            };

            self.add_line(format!("return converted.first{}", suffix));
        }

        self.add_closing_brackets();
    }
}

impl<'a> Deref for DynQueryParser<'a> {
    type Target = LineWriter;

    fn deref(&self) -> &Self::Target {
        &self.line_writer
    }
}

impl<'a> DerefMut for DynQueryParser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.line_writer
    }
}

fn separate_by_colon(parameters: &[(String, String)]) -> String {
    parameters
        .iter()
        .map(|(parameter_name, parameter_value)| format!("{}: {}", parameter_name, parameter_value))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Tests a query
fn test_query(connection: &Connection, query: &str) -> String {
    // use 1 for parameters: https://github.com/ballista-compute/sqlparser-rs/issues/291
    // Thanks to SQLite weak typing, all parameterized queries can be easily testing by executing it with '1'
    let query_for_validation = query.replace(" ?", " 1");

    println!("Validating query '{}'", query_for_validation);

    if let Err(e) = connection.query_row(&query_for_validation, NO_PARAMS, |_| Ok(())) {
        match e {
            Error::QueryReturnedNoRows => {
                // Fine
            }
            _ => panic!("Invalid query: {:#?}, error: {:#?}", query, e),
        }
    }

    query_for_validation
}

#[cfg(test)]
mod tests {
    use crate::dynamic_queries::parse::test_query;

    #[test]
    #[should_panic]
    fn test_query_fail() {
        let connection = rusqlite::Connection::open("ignoreme").unwrap();

        test_query(&connection, "select * from tablethatdoesnotexists");
    }
}
