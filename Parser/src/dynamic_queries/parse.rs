use crate::dynamic_queries::return_type::{Query, ReturnType};
use crate::line_writer::parameter_types_separated_colon;
use crate::parse::{test_query, Parser};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{create_swift_type_name, encode_swift_properties, SwiftProperty};
use grdb_orm_lib::dyn_query::DynamicQuery;

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
        self.add_line("import GRDBQuery".to_string());

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
            self.write_extension(dynamic_query);

            // Find out the return type
            let query = ReturnType {
                dynamic_query,
                line_writer: &mut self.line_writer,
                tables: self.tables,
                config: self.config,
                write_to_line_writer: true,
            }
            .parse();

            self.new_line();

            let func_return_type = if query.return_type().is_empty() {
                "".to_string()
            } else {
                format!("-> {}", query.return_type())
            };
            let parameter_types_separated_colons = parameter_types_separated_colon(
                &parameters.iter().map(|p| &p.1).collect::<Vec<_>>(),
            );

            // Write the method declaration
            self.add_with_modifier(format!(
                "static func {}(db: Database{}) throws {} {{",
                dynamic_query.func_name, &parameter_types_separated_colons, func_return_type
            ));

            self.write_body(dynamic_query, parameters.clone(), &query, &func_return_type);

            if let Some(different_type) = &dynamic_query.map_to_different_type {
                assert!(!func_return_type.is_empty());

                // Make sure the result type is the same
                let (mapped_to_type, func_name) = if different_type.contains('.') {
                    let split = different_type.split('.').collect::<Vec<_>>();

                    assert_eq!(2, split.len());

                    (split[0].to_string(), split[1].to_string())
                } else {
                    (dynamic_query.extension.clone(), different_type.clone())
                };

                // Search it in the list
                let matched = self
                    .config
                    .dynamic_queries
                    .iter()
                    .filter(|dyn_query| dyn_query.extension == mapped_to_type)
                    .filter(|dyn_query| dyn_query.func_name == func_name)
                    .collect::<Vec<_>>();

                assert_eq!(
                    1,
                    matched.len(),
                    "No match for mapped type: {}",
                    different_type
                );

                let dyn_query = matched[0];

                assert_eq!(dyn_query.return_types, dynamic_query.return_types);
                assert_eq!(
                    dyn_query.return_types_is_array,
                    dynamic_query.return_types_is_array
                );

                let mapped_type = ReturnType {
                    dynamic_query: dyn_query,
                    line_writer: &mut self.line_writer,
                    tables: self.tables,
                    config: self.config,
                    write_to_line_writer: false,
                }
                .parse();
                let mapped_return_type = format!("-> {}", mapped_type.return_type());

                self.add_with_modifier(format!(
                    "static func {}Mapped(db: Database{}) throws {} {{",
                    dynamic_query.func_name, &parameter_types_separated_colons, mapped_return_type
                ));

                self.write_body(
                    dynamic_query,
                    parameters.clone(),
                    &mapped_type,
                    &mapped_return_type,
                );
            }

            self.write_queryable_type(
                dynamic_query,
                &query,
                parameters.iter().map(|(_, b)| b).collect(),
            );

            self.add_closing_brackets();
        }

        self.write("DynamicQueries");
    }

    fn write_queryable_type(
        &mut self,
        dynamic_query: &DynamicQuery,
        query: &Query,
        parameters: Vec<&SwiftProperty>,
    ) {
        let the_type = match query {
            Query::Select { return_type: val } => val,
            Query::UpdateOrDelete => return,
        };

        let default_value = if dynamic_query.return_types_is_array {
            "[]"
        } else {
            "nil"
        };

        let modifier = self.modifier;
        let scheduler_parameter = "scheduler: ValueObservationScheduler = .async(onQueue: .main)";
        let assign_scheduler = "self.scheduler = scheduler";

        let (to_add, call_method, equatable) = if parameters.is_empty() {
            (
                format!(
                    "
                {modifier}init(
            {scheduler_parameter}
            ) {{
            {assign_scheduler}
            }}
            "
                ),
                "db: db".to_string(),
                "// TODO: not sure if this is correct\ntrue".to_string(),
            )
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
                .map(|p| format!("{modifier}let {p}"))
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
            {modifier}init(
            {init},
            {scheduler_parameter}
            ) {{
            {assign}
            {assign_scheduler}
            }}"
            );
            let equatable = parameters
                .iter()
                .map(|p| format!("lhs.{a} == rhs.{a}", a = p.swift_property_name))
                .collect::<Vec<_>>()
                .join(" && ");

            (extra_init, call_method, equatable)
        };

        self.add_comment("Very basic Queryable struct, create a PR if you want more customization");
        // Write the Queryable type
        self.add_with_modifier(format!(
            "struct {}Queryable: Queryable, Equatable {{
            {modifier}let scheduler: ValueObservationScheduler
            {to_add}
            {modifier}static let defaultValue: {the_type} = {default_value}

            {modifier}static func == (lhs: Self, rhs: Self) -> Bool {{
                {equatable}
            }}

            {modifier}func publisher(in dbQueue: DatabaseQueue) -> AnyPublisher<{the_type}, Error> {{
                    ValueObservation
                            .tracking({{ db in
                                try {}({call_method})
                            }})
                            .publisher(in: dbQueue, scheduling: scheduler)
                            .eraseToAnyPublisher()
                }}
            }}",
            some_kind_of_uppercase_first_letter(&dynamic_query.func_name),
            dynamic_query.func_name,
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

        self.add_line(
            "Logging.log(query)

        let statement = try db.cachedStatement(sql: query)",
        );

        if has_arguments {
            self.add_line("statement.setUncheckedArguments(arguments)");
        }

        match &query {
            Query::Select { return_type: _ } => {
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
                let decoding = format!("{return_value_closure}.init(row: row)");

                // Add the converted property
                self.add_line(format!(
                    "let converted: {type_fetch_all} = try Row.fetchAll(statement).map({{ row -> {return_value_closure} in
                        {decoding}
                    }})
                    "
                ));

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
