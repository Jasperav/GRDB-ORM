use crate::line_writer::StaticInstance;
use crate::query_writer::{write_static_queries, WriteResult};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{encode_swift_properties, SwiftProperty};
use crate::table_meta_data::TableMetaData;

pub const INSERT_UNIQUE_QUERY: &str = "insertUniqueQuery";
pub const INSERT_OR_IGNORE_QUERY: &str = "insertOrIgnoreUniqueQuery";
pub const REPLACE_UNIQUE_QUERY: &str = "replaceUniqueQuery";
pub const DELETE_ALL_QUERY: &str = "deleteAllQuery";
pub const UPDATE_UNIQUE_QUERY: &str = "updateUniqueQuery";
pub const SELECT_COUNT: &str = "selectCountQuery";

/// Writes the static queries for the main struct
pub struct QueryWriterMainStruct<'a> {
    table_meta_data: TableMetaData<'a>,
    non_pk: String,
}

impl<'a> QueryWriterMainStruct<'a> {
    pub fn new(table_meta_data: TableMetaData<'a>) -> Self {
        Self {
            non_pk: table_meta_data
                .non_primary_keys()
                .iter()
                .map(|p| format!("{} = ?", p.column.name))
                .collect::<Vec<_>>()
                .join(", "),
            table_meta_data,
        }
    }
}

fn create_upsert_query_name(column_name: &str) -> String {
    format!(
        "upsert{}Query",
        some_kind_of_uppercase_first_letter(column_name)
    )
}

// Static queries
impl<'a> QueryWriterMainStruct<'a> {
    pub(crate) fn write_static_queries(mut self) {
        let mut static_queries = vec![
            self.static_unique_insert_query(),
            self.static_unique_replace_query(),
            self.static_insert_or_ignore_query(),
            self.static_delete_all_query(),
            self.static_select_count_query(),
        ];

        if !self.non_pk.is_empty() {
            static_queries.push(self.static_unique_update_query())
        }

        // The first static query is always the insert query
        static_queries.extend_from_slice(&self.static_upsert_queries(&static_queries[0].1));

        write_static_queries(&mut self.table_meta_data.line_writer, static_queries);
    }

    fn static_upsert_queries(&self, insert_query: &str) -> Vec<WriteResult> {
        let mut v = vec![];
        let pk_comma_separated = self
            .table_meta_data
            .primary_keys()
            .iter()
            .map(|p| p.column.name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        for column in self.table_meta_data.non_primary_keys() {
            let query_name = create_upsert_query_name(&column.column.name);
            let query = format!(
                "{} on conflict ({}) do update set {column}=excluded.{column}",
                insert_query,
                pk_comma_separated,
                column = column.column.name
            );

            v.push((query_name, query));
        }

        v
    }

    fn static_delete_all_query(&mut self) -> WriteResult {
        (
            DELETE_ALL_QUERY.to_string(),
            format!("delete from {}", self.table_meta_data.table_name),
        )
    }

    fn static_select_count_query(&mut self) -> WriteResult {
        (
            SELECT_COUNT.to_string(),
            format!("select count(*) from {}", self.table_meta_data.table_name),
        )
    }

    fn columns_question_marks(&self, query: &'static str, prefix: &str) -> WriteResult {
        let separated_columns = self
            .table_meta_data
            .swift_properties
            .iter()
            .map(|c| c.column.name.clone())
            .collect::<Vec<_>>();
        let question_marks = separated_columns
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        (
            query.to_string(),
            format!(
                "{} into {} ({}) values ({})",
                prefix,
                self.table_meta_data.table_name,
                separated_columns.join(", "),
                question_marks
            ),
        )
    }

    pub fn static_unique_insert_query(&mut self) -> WriteResult {
        self.columns_question_marks(INSERT_UNIQUE_QUERY, "insert")
    }

    fn static_unique_replace_query(&mut self) -> WriteResult {
        self.columns_question_marks(REPLACE_UNIQUE_QUERY, "replace")
    }

    pub fn static_insert_or_ignore_query(&mut self) -> WriteResult {
        self.columns_question_marks(INSERT_OR_IGNORE_QUERY, "insert or ignore")
    }

    fn static_unique_update_query(&mut self) -> WriteResult {
        (
            UPDATE_UNIQUE_QUERY.to_string(),
            format!(
                "update {} set {} where {}",
                self.table_meta_data.table_name,
                self.non_pk,
                self.table_meta_data.primary_key_name_columns_separated()
            ),
        )
    }
}

// Methods
impl<'a> QueryWriterMainStruct<'a> {
    pub(crate) fn write_method(mut self) {
        self.write_insert();
        self.write_delete();
        self.write_update();
        self.write_updatable_columns();
        self.write_updatable_all();
        self.write_select_count();
    }

    /// Writes the static update queries that hits all rows in the table
    fn write_updatable_all(&mut self) {
        for column in self.table_meta_data.swift_properties {
            let capitalized_name = some_kind_of_uppercase_first_letter(&column.swift_property_name);
            let encoded = encode_swift_properties(&[column]);
            let query = format!(
                "\"update {} set {} = ?\"",
                self.table_meta_data.table_name, column.column.name
            );

            self.table_meta_data.line_writer.add_with_modifier(format!(
                "
                static func genUpdate{}AllRows(db: Database, {}: {}) throws {{
                    let arguments: StatementArguments = try [
                        {},
                    ]


                    let statement = try db.cachedUpdateStatement(sql: {})

                    statement.setUncheckedArguments(arguments)

                    try statement.execute()
                }}
            ",
                capitalized_name,
                column.swift_property_name,
                column.swift_type.type_name,
                encoded,
                query
            ));
        }
    }

    fn write_updatable_columns(&mut self) {
        let db_values = encode_swift_properties(
            self.table_meta_data
                .swift_properties
                .clone()
                .iter_mut()
                .map(|s| {
                    s.refers_to_self = true;

                    // lol https://stackoverflow.com/a/41367094/7715250
                    &*s
                })
                .collect::<Vec<_>>()
                .as_slice(),
        );

        // Write the updatable columns
        let updatable_columns = self.table_meta_data.swift_properties.to_vec();

        // The ref makes it easier to call other functions
        let updatable_columns = updatable_columns.iter().collect::<Vec<_>>();

        assert!(!updatable_columns.is_empty());

        let pk_separated = self.table_meta_data.primary_key_name_columns_separated();
        let cases = updatable_columns
            .iter()
            .map(|t| t.swift_property_name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        let mut update_queries = vec![];

        let update = "update";

        for column in &updatable_columns {
            let update_query = format!(
                "update {} set {} = ? where {}",
                self.table_meta_data.table_name, column.column.name, pk_separated
            );

            update_queries.push(format!(
                "{} static let {}{}Query = \"{}\"\n",
                self.table_meta_data.line_writer.modifier,
                update,
                some_kind_of_uppercase_first_letter(&column.swift_property_name),
                update_query
            ));
        }

        self.table_meta_data.line_writer.add_with_modifier(format!(
            "enum UpdatableColumn: String {{
                case {}

                {}
             }}",
            cases,
            update_queries.join(""),
        ));

        let cases_with_associated_values = updatable_columns
            .iter()
            .map(|t| {
                let pn = t.swift_property_name.clone();
                let value = t.swift_type.type_name.clone();

                format!("{}({})", pn, value)
            })
            .collect::<Vec<_>>()
            .join(", ");

        let column_name = updatable_columns
            .iter()
            .map(|sp| {
                format!(
                    "case .{}: return \"{}\"",
                    sp.swift_property_name, sp.swift_property_name
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Write the enum with the associated values
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "enum UpdatableColumnWithValue {{
                case {}

                var columnName: String {{
                    switch self {{
                        {}
                    }}
                }}
             }}",
            cases_with_associated_values, column_name
        ));

        // Create a simple mapper between struct and the enum
        for sp in &updatable_columns {
            let column_name = some_kind_of_uppercase_first_letter(&sp.swift_property_name);
            let transformed = if sp.serialize_deserialize_blob(false).is_some() {
                format!("{}AutoConvert()", sp.swift_property_name)
            } else {
                sp.swift_property_name.clone()
            };

            self.table_meta_data.line_writer.add_with_modifier(format!(
                "
                func createColumn{}() -> Self.UpdatableColumnWithValue {{
                    return .{}({})
                }}
            ",
                column_name, sp.swift_property_name, transformed
            ))
        }

        // Create a comma separated string of primary keys, used for ON CONFLICT clause in the query
        let pk_comma = self
            .table_meta_data
            .primary_keys()
            .into_iter()
            .map(|t| t.column.name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        let switch = updatable_columns
            .iter()
            .map(|t| {
                format!(
                    "case .{name}:
                        if processedAtLeastOneColumns {{
                            upsertQuery += \", \"
                        }}
                        upsertQuery += \"{name}=excluded.{name}\"\
                    ",
                    name = t.swift_property_name
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Write the dynamic upsert method
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "func genUpsertDynamic(db: Database, columns: [UpdatableColumn], assertAtLeastOneUpdate: Bool = true) throws {{
                assert(!assertAtLeastOneUpdate || !columns.isEmpty)

                // Check for duplicates
                assert(Set(columns).count == columns.count)

                if columns.isEmpty {{
                    return
                }}

                var upsertQuery = {}.insertUniqueQuery + \"on conflict ({}) do update set \"
                var processedAtLeastOneColumns = false

                for column in columns {{
                    switch column {{
                        {}
                    }}

                    processedAtLeastOneColumns = true
                }}

                let arguments: StatementArguments = try [
                    {}
                ]

                let statement = try db.cachedUpdateStatement(sql: upsertQuery)

                statement.setUncheckedArguments(arguments)

                try statement.execute()
            }}
            ",
            self.table_meta_data.struct_name,
            pk_comma,
            switch,
            db_values,
        ));
    }

    fn write_select_count(&mut self) {
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "
            static func genSelectCount(db: Database) throws -> Int {{
                let statement = try db.cachedSelectStatement(sql: {})

                return try Int.fetchOne(statement)!
            }}
        ",
            SELECT_COUNT
        ));
    }

    fn write_insert(&mut self) {
        let db_values = encode_swift_properties(
            self.table_meta_data
                .swift_properties
                .iter()
                .collect::<Vec<_>>()
                .as_slice(),
        );

        self.write("Insert", INSERT_UNIQUE_QUERY, &db_values, true);
        self.write("InsertOrIgnore", INSERT_OR_IGNORE_QUERY, &db_values, false);
        self.write("Replace", REPLACE_UNIQUE_QUERY, &db_values, false);

        let non_pk_cloned = self
            .table_meta_data
            .non_primary_keys()
            .iter()
            .map(|c| <&SwiftProperty>::clone(c).clone())
            .collect::<Vec<_>>();

        for column in non_pk_cloned {
            let query_name = create_upsert_query_name(&column.column.name);
            let method_name = query_name.strip_suffix("Query").unwrap().to_string();

            self.write(
                &some_kind_of_uppercase_first_letter(&method_name),
                &query_name,
                &db_values,
                false,
            );
        }
    }

    fn write_delete(&mut self) {
        self.write("DeleteAll", DELETE_ALL_QUERY, "", true);
    }

    fn write_update(&mut self) {
        let mut non_pk = self.table_meta_data.non_primary_keys();

        if non_pk.is_empty() {
            return;
        }

        let mut pk = self.table_meta_data.primary_keys();

        non_pk.append(&mut pk);

        let values = encode_swift_properties(&non_pk);

        self.write("Update", UPDATE_UNIQUE_QUERY, &values, true);
    }

    fn write(&mut self, method_name: &str, query: &str, values: &str, add_check: bool) {
        let (static_instance, args, check, arguments) = if values.is_empty() {
            (StaticInstance::Static, "".to_string(), "", "")
        } else {
            let args = format!(
                "let arguments: StatementArguments = try [
                    {}
                ]

                statement.setUncheckedArguments(arguments)",
                values
            );
            let (check, argument) = if add_check {
                ("if assertOneRowAffected {\n// Only 1 row should be affected\nassert(db.changesCount == 1)}", ", assertOneRowAffected: Bool = true")
            } else {
                ("", "")
            };

            (StaticInstance::Instance, args, check, argument)
        };

        self.table_meta_data.line_writer.add_with_modifier(format!(
            "{}func gen{}(db: Database{}) throws {{
                let statement = try db.cachedUpdateStatement(sql: Self.{})

                {}

                try statement.execute()

                {}
            }}
        ",
            static_instance.modifier(),
            method_name,
            arguments,
            query,
            args,
            check
        ));
    }
}
