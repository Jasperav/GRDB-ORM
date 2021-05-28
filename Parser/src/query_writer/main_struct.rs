use crate::line_writer::StaticInstance;
use crate::query_writer::{write_static_queries, WriteResult};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::encode_swift_properties;
use crate::table_meta_data::TableMetaData;

pub const INSERT_UNIQUE_QUERY: &str = "insertUniqueQuery";
pub const INSERT_OR_IGNORE_QUERY: &str = "insertOrIgnoreUniqueQuery";
pub const REPLACE_UNIQUE_QUERY: &str = "replaceUniqueQuery";
pub const DELETE_ALL_QUERY: &str = "deleteAllQuery";
pub const UPDATE_UNIQUE_QUERY: &str = "updateUniqueQuery";

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
        some_kind_of_uppercase_first_letter(&column_name)
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

        return v;
    }

    fn static_delete_all_query(&mut self) -> WriteResult {
        (
            DELETE_ALL_QUERY.to_string(),
            format!("delete from {}", self.table_meta_data.table_name),
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
            .map(|c| c.clone().clone())
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
        self.write("DeleteAll", DELETE_ALL_QUERY, &"", true);
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
