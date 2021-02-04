use crate::query_writer::{write_static_queries, WriteResult};
use crate::swift_property::swift_properties_to_sqlite_database_values;
use crate::table_meta_data::TableMetaData;

pub const SELECT_QUERY: &str = "select_query";
pub const DELETE_QUERY: &str = "delete_query";

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

        self.table_meta_data.line_writer.add_closing_brackets();
    }

    fn write_select_query(&mut self) {
        let values =
            swift_properties_to_sqlite_database_values(self.table_meta_data.primary_keys());

        self.table_meta_data
            .line_writer
            .add_comment("Queries a unique row in the database, the row may or may not exist");
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "func genSelect(db: Database) throws -> {}? {{
            let statement = try db.cachedSelectStatement(sql: Self.{})

            statement.setUncheckedArguments(StatementArguments(values: [
            {}
            ]))

            return try {}.fetchOne(statement)
        }}
        ",
            self.table_meta_data.struct_name,
            SELECT_QUERY,
            values,
            self.table_meta_data.struct_name
        ))
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
        ))
    }

    fn write_delete_query(&mut self) {
        let values =
            swift_properties_to_sqlite_database_values(self.table_meta_data.primary_keys());

        self.table_meta_data
            .line_writer
            .add_comment("Deletes a unique row, asserts that the row actually existed");
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "func genDelete(db: Database) throws {{
            let values = [
                {}
            ]

            let statement = try db.cachedUpdateStatement(sql: Self.{})

            statement.setUncheckedArguments(StatementArguments(values: values))

            try statement.execute()

            assert(db.changesCount == 1)
        }}
        ",
            values, DELETE_QUERY
        ))
    }
}
