use crate::dynamic_queries::reader::DynamicQuery;
use crate::dynamic_queries::return_type::{Query, ReturnType};
use crate::line_writer::{parameter_types_separated_colon, StaticInstance, WriteRead};
use crate::parse::{test_query, Parser};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::create_swift_type_name;

/// Parses a dynamic query
impl<'a> Parser<'a> {
    /// Parses the dynamic queries
    pub fn parse_dyn_queries(mut self) {
        if self.config.dynamic_queries.is_empty() {
            println!("No dynamic queries found");

            return;
        }

        for dynamic_query in &self.config.dynamic_queries {
            // Check if the query is valid
            test_query(
                &self.config.sqlite_location,
                &dynamic_query.query,
                dynamic_query.return_types.is_empty(),
            );

            // The parameters to invoke the Swift functions for
            let mut parameters = vec![];

            // The conversion of parameter to databaseValue
            let mut database_values = vec![];

            for (table, column, param_name) in &dynamic_query.parameter_types {
                self.find_swift_property(
                    table,
                    column,
                    param_name,
                    &mut database_values,
                    &mut parameters,
                );
            }

            let parameters = parameters.iter().collect::<Vec<_>>();

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

            self.new_line();

            let func_return_type = if query.return_type().is_empty() {
                "".to_string()
            } else {
                format!("-> {}", query.return_type())
            };

            // Write the method declaration
            self.add_with_modifier(format!(
                "static func {}(db: Database{}) throws {} {{",
                dynamic_query.func_name,
                parameter_types_separated_colon(&parameters),
                func_return_type
            ));

            self.write_body(&dynamic_query, database_values, &query);
            let read_write = match query {
                Query::Select { .. } => WriteRead::Read(query.return_type()),
                Query::UpdateOrDelete => WriteRead::Write,
            };

            self.line_writer.add_wrapper_pool(
                StaticInstance::Static,
                &dynamic_query.func_name,
                read_write,
                false,
                &parameters,
            );

            self.add_closing_brackets();
        }

        self.write("DynamicQueries");
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
                let return_value =
                    query.replace_optional_for_closure(dynamic_query.return_types_is_array);

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
                self.add_line("})");

                if dynamic_query.return_types_is_array {
                    // if the return type is an array, it can be returned directly, no need for checking the resultset
                    self.add_line("return converted");
                } else {
                    self.add_line("assert(converted.count <= 1, \"Expected 1 or zero rows\")");
                    self.add_line("return converted.first");
                }
            }
            Query::UpdateOrDelete => {
                // This is easy, just execute it
                self.add_line("try statement.execute()");
            }
        }

        self.add_closing_brackets();
    }
}
