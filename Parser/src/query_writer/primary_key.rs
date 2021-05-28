use crate::query_writer::{write_static_queries, WriteResult};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{encode_swift_properties, SwiftProperty};
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
        let values = encode_swift_properties(&self.table_meta_data.primary_keys());

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
    }

    fn execute_update_statement(
        &mut self,
        fn_name: &str,
        parameters: &[&SwiftProperty],
        values: &[&SwiftProperty],
        sql: &str,
        add_assert_one_row_affected: bool,
    ) {
        self.table_meta_data.write_update(
            fn_name,
            parameters,
            values,
            sql,
            true,
            add_assert_one_row_affected,
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
            .cloned()
            .collect::<Vec<_>>();

        assert!(!values.is_empty());

        self.execute_update_statement(
            "Delete",
            &[],
            &values.iter().collect::<Vec<_>>(),
            &format!("Self.{}", DELETE_QUERY),
            true,
        )
    }

    fn write_updatable_columns(&mut self) {
        // Write the updatable columns
        let updatable_columns = self.table_meta_data.swift_properties.to_vec();

        // The ref makes it easier to call other functions
        let updatable_columns = updatable_columns.iter().collect::<Vec<_>>();

        assert!(!updatable_columns.is_empty());

        let pk_separated = self.table_meta_data.primary_key_name_columns_separated();
        let pk_comma_separated = self
            .table_meta_data
            .primary_keys()
            .iter()
            .map(|p| p.column.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        let cases = updatable_columns
            .iter()
            .map(|t| t.swift_property_name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        let mut update_queries = vec![];
        let mut upsert_queries = vec![];

        let update = "update";
        let upsert = "upsert";

        for column in &updatable_columns {
            let update_query = format!(
                "update {} set {} = ? where {}",
                self.table_meta_data.table_name, column.column.name, pk_separated
            );
            let create_query = |prefix, query| {
                format!(
                    "{} static let {}{}Query = \"{}\"\n",
                    self.table_meta_data.line_writer.modifier,
                    prefix,
                    some_kind_of_uppercase_first_letter(&column.swift_property_name),
                    query
                )
            };

            update_queries.push(create_query(update, update_query.clone()));

            // Doesn't really makes sense
            if !column.column.part_of_pk {
                let upsert_postfix = format!(
                    " on conflict({}) do update set {column}=excluded.{column}",
                    pk_comma_separated,
                    column = column.column.name
                );

                let upsert_query = format!("{}{}", update_query, upsert_postfix);

                upsert_queries.push(create_query(upsert, upsert_query));
            }
        }

        self.table_meta_data.line_writer.add_with_modifier(format!(
            "enum UpdatableColumn {{
                case {}

                {}\n
                {}
             }}",
            cases,
            update_queries.join(""),
            upsert_queries.join(""),
        ));

        for property in &updatable_columns {
            let mut values = self
                .table_meta_data
                .primary_keys()
                .into_iter()
                .cloned()
                .map(|mut s| {
                    s.refers_to_self = true;

                    s
                })
                .collect::<Vec<_>>();

            values.insert(0, <&SwiftProperty>::clone(property).clone());

            let mut execute_update = |update, add_assert_one_row_affected, sql: String| {
                self.execute_update_statement(
                    &format!(
                        "{}{}",
                        some_kind_of_uppercase_first_letter(update),
                        some_kind_of_uppercase_first_letter(&property.swift_property_name)
                    ),
                    &[property],
                    &values.iter().collect::<Vec<_>>(),
                    &sql,
                    add_assert_one_row_affected,
                );
            };

            let query_name = |prefix: &str| {
                format!(
                    "UpdatableColumn.{}{}Query",
                    prefix,
                    some_kind_of_uppercase_first_letter(&property.swift_property_name),
                )
            };

            execute_update(update, true, query_name(update));

            if !property.column.part_of_pk {
                execute_update(upsert, false, query_name(upsert));
            }
        }
    }
}
