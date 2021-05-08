use crate::line_writer::{parameter_types_separated_colon, StaticInstance, WriteRead};
use crate::query_writer::{write_static_queries, WriteResult};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{swift_properties_to_sqlite_database_values, SwiftProperty};
use crate::table_meta_data::TableMetaData;

pub const SELECT_QUERY: &str = "selectQuery";
pub const DELETE_QUERY: &str = "deleteQuery";

/// Writes the unique queries for the primary key
pub struct QueryWriterPrimaryKey<'a> {
    pub table_meta_data: TableMetaData<'a>,
}

// Static queries
impl<'a> QueryWriterPrimaryKey<'a> {
    pub(crate) fn write_static_queries(mut self) {
        let static_queries = vec![self.static_select_query(), self.static_delete_query()];

        write_static_queries(&mut self.table_meta_data.line_writer, static_queries);
    }

    fn static_select_query(&mut self) -> WriteResult {
        (
            SELECT_QUERY,
            format!(
                "select * from {}{}",
                self.table_meta_data.table_name,
                self.table_meta_data.where_clause()
            ),
        )
    }

    fn static_delete_query(&mut self) -> WriteResult {
        (
            DELETE_QUERY,
            format!(
                "delete from {}{}",
                self.table_meta_data.table_name,
                self.table_meta_data.where_clause()
            ),
        )
    }
}

// Methods
impl<'a> QueryWriterPrimaryKey<'a> {
    pub(crate) fn write_method(mut self) {
        self.write_select_query();
        self.write_select_query_expect();
        self.write_delete_query();
        self.write_updatable_columns();

        self.table_meta_data.line_writer.add_closing_brackets();
    }

    fn write_select_query(&mut self) {
        let values =
            swift_properties_to_sqlite_database_values(&self.table_meta_data.primary_keys());

        self.table_meta_data
            .line_writer
            .add_comment("Queries a unique row in the database, the row may or may not exist");
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "func genSelect(db: Database) throws -> {}? {{
            let arguments: StatementArguments = try [
                {}
            ]

            let statement = try db.cachedSelectStatement(sql: Self.{})

            statement.setUncheckedArguments(arguments)

            return try {}.fetchOne(statement)
        }}
        ",
            self.table_meta_data.struct_name,
            values,
            SELECT_QUERY,
            self.table_meta_data.struct_name
        ));
        self.table_meta_data.line_writer.add_wrapper_pool(
            StaticInstance::Instance,
            "Select",
            WriteRead::Read(format!("{}?", self.table_meta_data.struct_name)),
            true,
            &[],
        );
    }

    fn write_select_query_expect(&mut self) {
        self.table_meta_data.line_writer.add_comment(
            "Same as function 'genSelectUnique', but throws an error when no record has been found",
        );
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "func genSelectExpect(db: Database) throws -> {} {{
            if let instance = try genSelect(db: db) {{
                return instance
            }} else {{
                throw DatabaseError(message: \"Didn't found a record for \\(self)\")
            }}
        }}
        ",
            self.table_meta_data.struct_name
        ));

        self.table_meta_data.line_writer.add_wrapper_pool(
            StaticInstance::Instance,
            "SelectExpect",
            WriteRead::Read(self.table_meta_data.struct_name.to_string()),
            true,
            &[],
        );
    }

    fn execute_update_statement(
        &mut self,
        fn_name: &str,
        parameters: &[&SwiftProperty],
        values: &[&SwiftProperty],
        statement: &str,
    ) {
        self.table_meta_data.write_update_with_wrapper(
            fn_name,
            parameters,
            values,
            &format!("Self.{}", statement),
            true,
        );
    }

    fn write_delete_query(&mut self) {
        self.table_meta_data
            .line_writer
            .add_comment("Deletes a unique row, asserts that the row actually existed");

        let values = self
            .table_meta_data
            .primary_keys()
            .into_iter()
            .map(|s| s.clone())
            .collect::<Vec<_>>();

        assert!(!values.is_empty());

        self.execute_update_statement(
            "Delete",
            &[],
            &values.iter().collect::<Vec<_>>(),
            DELETE_QUERY,
        )
    }

    fn write_updatable_columns(&mut self) {
        // Write the updatable columns
        let updatable_columns = self
            .table_meta_data
            .non_primary_keys()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        // The ref makes it easier to call other functions
        let updatable_columns = updatable_columns.iter().collect::<Vec<_>>();

        if updatable_columns.is_empty() {
            return;
        }

        let pk_separated = self.table_meta_data.primary_key_name_columns_separated();
        let cases = updatable_columns
            .iter()
            .map(|t| t.swift_property_name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        let queries = updatable_columns
            .iter()
            .map(|u| {
                let update_query = format!(
                    "update {} set {} = ? where {}",
                    self.table_meta_data.table_name, u.column.name, pk_separated
                );
                format!(
                    "{} static let update{}Query = \"{}\"",
                    self.table_meta_data.line_writer.modifier,
                    some_kind_of_uppercase_first_letter(&u.swift_property_name),
                    update_query
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        self.table_meta_data.line_writer.add_with_modifier(format!(
            "enum UpdatableColumn {{
                case {}

                {}
             }}",
            cases, queries
        ));

        for property in &updatable_columns {
            let mut values = self
                .table_meta_data
                .primary_keys()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>();

            values.insert(0, <&SwiftProperty>::clone(property).clone());

            self.execute_update_statement(
                &format!(
                    "Update{}",
                    some_kind_of_uppercase_first_letter(&property.swift_property_name)
                ),
                &[property],
                &values.iter().collect::<Vec<_>>(),
                &format!(
                    "UpdatableColumn.update{}Query",
                    some_kind_of_uppercase_first_letter(&property.swift_property_name)
                ),
            );
        }
    }
}
