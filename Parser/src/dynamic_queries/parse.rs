use std::ops::{Deref, DerefMut};

use rusqlite::{Connection, Error, NO_PARAMS};
use sqlite_parser::Metadata;

use crate::configuration::Config;
use crate::dynamic_queries::reader::DynamicQuery;
use crate::dynamic_queries::return_type::{Query, ReturnType};
use crate::line_writer::{LineWriter, WriteRead};
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
            test_query(
                &connection,
                &dynamic_query.query,
                dynamic_query.return_types.is_empty(),
            );

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
                    database_values.push(param_name.clone());
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
            let query = ReturnType {
                return_types: &dynamic_query.return_types,
                return_type_is_array: dynamic_query.return_types_is_array,
                tables: self.tables,
                config: self.config,
            }
            .parse();

            self.write_extension(&dynamic_query);
            let capitalized_func_name =
                some_kind_of_uppercase_first_letter(&dynamic_query.func_name);

            query.write_type_alias(&mut self, &capitalized_func_name);

            let mut parameters_main = parameters.clone();

            // The db is always the first parameter
            parameters_main.insert(0, ("db".to_string(), "Database".to_string()));

            self.new_line();

            let func_return_type = if query.return_type().is_empty() {
                "".to_string()
            } else {
                format!("-> {}", query.return_type())
            };

            // Write the method declaration
            self.add_line(format!(
                "static func {}({}) throws {} {{",
                dynamic_query.func_name,
                separate_by_colon(&parameters_main),
                func_return_type
            ));

            self.write_body(&dynamic_query, database_values, &query);

            match query {
                Query::Select { .. } => {
                    self.write_easy_method(
                        WriteRead::Read(query.return_type()),
                        &dynamic_query.func_name,
                        &parameters,
                    );
                }
                Query::UpdateOrDelete => {
                    self.write_easy_method(WriteRead::Write, &dynamic_query.func_name, &parameters);
                }
            }

            self.add_closing_brackets();
        }

        self.line_writer.write_to_file("DynamicQueries");

        println!("Successfully processed all dynamic queries");
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

    #[allow(clippy::ptr_arg)] // Code doesn't compile with this lint
    fn write_easy_method(
        &mut self,
        write_read: WriteRead,
        func_name: &str,
        arguments: &Vec<(String, String)>,
    ) {
        let mut arguments_with_db = arguments.clone();

        // Add a generic parameter constraint on DatabaseReader, to maximize support
        arguments_with_db.insert(
            0,
            (
                write_read.database_reader_or_writer().to_string(),
                "T".to_string(),
            ),
        );

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
            "static func {}<T: {}>({}) throws{} {{",
            func_name,
            write_read.generic_type(),
            separate_by_colon(&arguments_with_db),
            write_read.return_type(),
        ));
        self.add_line(format!(
            "try {}.{} {{ db in
            try Self.{}(db: db{})
        }}",
            write_read.database_reader_or_writer(),
            write_read.to_str(),
            func_name,
            arguments_invocation
        ));
        self.add_closing_brackets();
    }

    /// Writes the actual body of the query processing
    fn write_body(
        &mut self,
        dynamic_query: &DynamicQuery,
        database_values: Vec<String>,
        query: &Query,
    ) {
        // Add the query as multiline text
        self.add_line(format!(
            "let statement = try db.cached{}Statement(sql: \"\"\"\n{}\n\"\"\")",
            query.statement(),
            dynamic_query.query
        ));

        if !database_values.is_empty() {
            // Set unchecked arguments to the statement if there are arguments
            let separated = database_values.join(", ");

            self.add_line(format!(
                "let arguments: StatementArguments = try [
                    {}
                ]

                statement.setUncheckedArguments(arguments)",
                separated
            ));
        }

        match &query {
            Query::Select {
                return_type: _return_type,
                decoding,
            } => {
                let return_value = query.replace_optional_for_closure(
                    dynamic_query.return_types_is_array,
                    &dynamic_query.return_types,
                );

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
                    (return_value.as_str(), format!("[{}]", return_value))
                };

                // Add the converted property
                self.add_line(format!(
                    "let converted: {} = try Row.fetchAll(statement).map({{ row -> {} in",
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
                        self.add_line(
                            "assert(converted.count == 1, \"Expected 1 row\")".to_string(),
                        );

                        // Not optional return type, force unwrap
                        "!"
                    };

                    self.add_line(format!("return converted.first{}", suffix));
                }
            }
            Query::UpdateOrDelete => {
                // This is easy, just execute it
                self.add_line("try statement.execute()".to_string());
            }
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
fn test_query(connection: &Connection, query: &str, return_types_is_empty: bool) -> String {
    // use 1 for parameters: https://github.com/ballista-compute/sqlparser-rs/issues/291
    // Thanks to SQLite weak typing, all parameterized queries can be easily testing by executing it with '1'
    let query_for_validation = query.replace(" ?", " 1");

    println!("Validating query '{}'", query_for_validation);

    // Check if the query starts with select, delete or update. Insert and anything else are illegal
    // This is because insert queries are already generated
    let lowercased = query.to_lowercase();

    if !lowercased.starts_with("update ")
        && !lowercased.starts_with("select ")
        && !lowercased.starts_with("delete from ")
    {
        panic!("Query should start with update, select or delete from");
    }

    if lowercased.starts_with("select ") {
        assert!(!return_types_is_empty)
    } else {
        assert!(return_types_is_empty)
    }

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

        test_query(&connection, "select * from tablethatdoesnotexists", false);
    }
}
