use crate::line_writer::{StaticInstance, WriteRead};
use crate::query_writer::{write_static_queries, WriteResult};
use crate::swift_property::swift_properties_to_sqlite_database_values;
use crate::table_meta_data::TableMetaData;

pub const INSERT_UNIQUE_QUERY: &str = "insertUniqueQuery";
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

// Static queries
impl<'a> QueryWriterMainStruct<'a> {
    pub(crate) fn write_static_queries(mut self) {
        let mut static_queries = vec![
            self.static_unique_insert_query(),
            self.static_delete_all_query(),
        ];

        if !self.non_pk.is_empty() {
            static_queries.push(self.static_unique_update_query())
        }

        write_static_queries(&mut self.table_meta_data.line_writer, static_queries);
    }

    fn static_delete_all_query(&mut self) -> WriteResult {
        (
            DELETE_ALL_QUERY,
            format!("delete from {}", self.table_meta_data.table_name),
        )
    }

    fn static_unique_insert_query(&mut self) -> WriteResult {
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
            INSERT_UNIQUE_QUERY,
            format!(
                "insert into {} ({}) values ({})",
                self.table_meta_data.table_name,
                separated_columns.join(", "),
                question_marks
            ),
        )
    }

    fn static_unique_update_query(&mut self) -> WriteResult {
        (
            UPDATE_UNIQUE_QUERY,
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

        self.table_meta_data.line_writer.add_closing_brackets();
    }

    fn write_insert(&mut self) {
        let db_values = swift_properties_to_sqlite_database_values(
            self.table_meta_data.swift_properties.iter().collect(),
        );

        self.write("Insert", INSERT_UNIQUE_QUERY, &db_values);
    }

    fn write_delete(&mut self) {
        self.write("DeleteAll", DELETE_ALL_QUERY, &"");
    }

    fn write_update(&mut self) {
        let mut non_pk = self.table_meta_data.non_primary_keys();

        if non_pk.is_empty() {
            return;
        }

        let mut pk = self.table_meta_data.primary_keys();

        non_pk.append(&mut pk);

        let values = swift_properties_to_sqlite_database_values(non_pk);

        self.write("Update", UPDATE_UNIQUE_QUERY, &values);
    }

    fn write(&mut self, method_name: &str, query: &str, values: &str) {
        let (static_instance, args, check) = if values.is_empty() {
            (StaticInstance::Static, "".to_string(), "".to_string())
        } else {
            let args = format!(
                "let arguments: StatementArguments = try [
                    {}
                ]

                statement.setUncheckedArguments(arguments)",
                values
            );
            let check = "// Only 1 row should be affected
                assert(db.changesCount == 1)";

            (StaticInstance::Instance, args, check.to_string())
        };

        self.table_meta_data.line_writer.add_with_modifier(format!(
            "{}func gen{}(db: Database) throws {{
                let statement = try db.cachedUpdateStatement(sql: Self.{})

                {}

                try statement.execute()

                {}
            }}
        ",
            static_instance.modifier(),
            method_name,
            query,
            args,
            check
        ));

        self.table_meta_data.line_writer.add_wrapper_pool(
            static_instance,
            method_name,
            "",
            WriteRead::Write,
        );
    }
}
