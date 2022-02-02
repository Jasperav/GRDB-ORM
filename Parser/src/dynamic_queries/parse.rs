use crate::dynamic_queries::reader::DynamicQuery;
use crate::dynamic_queries::return_type::{Query, ReturnType};
use crate::line_writer::parameter_types_separated_colon;
use crate::parse::{test_query, Parser};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{create_swift_type_name, encode_swift_properties, SwiftProperty};

pub const PARAMETERIZED_IN_QUERY: &str = "%PARAM_IN%";

/// Parses a dynamic query
impl<'a> Parser<'a> {
    /// Parses the dynamic queries
    pub fn parse_dyn_queries(mut self) {
        if self.config.dynamic_queries.is_empty() {
            println!("No dynamic queries found");

            return;
        }

        self.add_line("import Combine".to_string());

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

            let mut prefix = 0;

            for (table, column, param_name) in &dynamic_query.parameter_types {
                let default = usize::MAX;
                let current_match = dynamic_query.query[prefix..].find('?').unwrap_or(default);
                let in_query = dynamic_query.query[prefix..]
                    .find(PARAMETERIZED_IN_QUERY)
                    .unwrap_or(default);
                let belongs_to_parameterized_in_query = in_query < current_match;

                prefix = prefix
                    + if belongs_to_parameterized_in_query {
                        in_query
                    } else {
                        current_match
                    }
                    + 1;

                let mut swift_property =
                    self.find_swift_property(table, column, param_name, &mut database_values);

                if belongs_to_parameterized_in_query {
                    swift_property.swift_type.type_name =
                        format!("[{}]", swift_property.swift_type.type_name);
                }

                parameters.push((belongs_to_parameterized_in_query, swift_property));
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

            self.write_extension(dynamic_query);
            let capitalized_func_name =
                some_kind_of_uppercase_first_letter(&dynamic_query.func_name);

            let type_alias_with_type = query.write_type_alias(&mut self, &capitalized_func_name);

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
                parameter_types_separated_colon(
                    &parameters.iter().map(|p| &p.1).collect::<Vec<_>>()
                ),
                func_return_type
            ));

            self.write_body(dynamic_query, parameters.clone(), &query, &func_return_type);
            self.write_queryable_type(
                dynamic_query,
                type_alias_with_type,
                parameters.iter().map(|(_, b)| b).collect(),
            );

            self.add_closing_brackets();
        }

        self.write("DynamicQueries");
    }

    fn write_queryable_type(
        &mut self,
        dynamic_query: &DynamicQuery,
        type_alias_with_type: (String, String),
        parameters: Vec<&SwiftProperty>,
    ) {
        let (type_alias, the_type) = type_alias_with_type;

        if type_alias.is_empty() {
            return;
        }

        assert!(!the_type.is_empty());

        let default_value = if the_type.ends_with('?') {
            "nil"
        } else if the_type.ends_with(']') {
            "[]"
        } else {
            println!("Could not determine default value for query, therefore there is no Queryable type: {}", dynamic_query.query);

            return;
        };

        let (to_add, call_method) = if parameters.is_empty() {
            ("".to_string(), "db: db".to_string())
        } else {
            let property_type = parameters
                .iter()
                .map(|p| format!("{}: {}", p.swift_property_name, p.swift_type.type_name))
                .collect::<Vec<_>>();
            let init = property_type.join(",\n");
            let assign = parameters
                .iter()
                .map(|p| format!("self.{t} = {t}", t = p.swift_property_name))
                .collect::<Vec<_>>()
                .join("\n");
            let properties = property_type
                .iter()
                .map(|p| format!("public let {p}"))
                .collect::<Vec<_>>()
                .join("\n");
            let call_method = "db: db, ".to_string()
                + &parameters
                    .iter()
                    .map(|p| format!("{n}: {n}", n = p.swift_property_name))
                    .collect::<Vec<_>>()
                    .join(", ");
            let extra_init = format!(
                "{properties}
            public init(
            {init}
            ) {{
            {assign}
            }}"
            );

            (extra_init, call_method)
        };

        let modifier = self.modifier;

        self.add_comment("Very basic Queryable struct, create a PR if you want more customization");
        // Write the Queryable type
        self.add_with_modifier(format!(
            "struct {}Queryable: Queryable {{
            {to_add}
            static let defaultValue: {type_alias} = {default_value}

            {modifier}func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<{type_alias}, Error> {{
                    ValueObservation
                            .tracking({{ db in
                                try {}({call_method})
                            }})
                            .publisher(in: dbQueue)
                            .eraseToAnyPublisher()
                }}
            }}",
            some_kind_of_uppercase_first_letter(&dynamic_query.func_name),
            dynamic_query.func_name,
            type_alias = type_alias,
        ));
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
        swift_properties: Vec<&(bool, SwiftProperty)>,
        query: &Query,
        func_return_type: &str,
    ) {
        let has_arguments = !swift_properties.is_empty();

        self.add_line(format!(
            "var query = \"\"\"\n{}\n\"\"\"",
            dynamic_query.query
        ));

        if has_arguments {
            self.add_line("var arguments = StatementArguments()");
        }

        for (is_array, swift_property) in swift_properties {
            if *is_array {
                let mut swift_property_clone = swift_property.clone();

                swift_property_clone.swift_property_name = "v".to_string();

                let encode = encode_swift_properties(&[&swift_property_clone]);

                self.add_line(format!(
                    "if {param}.isEmpty {{
                        return {}
                    }}

                    for v in {param} {{
                        arguments += [{}]
                    }}

                    // Extra identifier is needed because else swift-format will format it incorrectly causing a compile error
                    _ = {{
                        let questionMarks = String(repeating: \"?, \", count: {param}.count)
                        // Remove the trailing question mark
                        let questionMarksCorrected = \"(\" + questionMarks.dropLast().dropLast() + \")\"
                        let occurrence = query.range(of: \"{}\")!

                        query = query.replacingCharacters(in: occurrence, with: questionMarksCorrected)
                    }}()
                    ",
                    if func_return_type.is_empty() {
                        ""
                    } else {
                        "[]"
                    },
                    encode,
                    PARAMETERIZED_IN_QUERY,
                    param = swift_property.swift_property_name,
                ));
            } else {
                let encode = encode_swift_properties(&[swift_property]);

                self.add_line(format!("arguments += [{}]", encode));
            }
        }

        self.add_line("let statement = try db.cachedStatement(sql: query)");

        if has_arguments {
            self.add_line("statement.setUncheckedArguments(arguments)");
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
