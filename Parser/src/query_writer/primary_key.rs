use crate::SET_ARGUMENTS;
use crate::query_writer::{WriteResult, write_static_queries};
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{SwiftProperty, encode_swift_properties};
use crate::table_meta_data::TableMetaData;

pub const SELECT_QUERY: &str = "selectQuery";
pub const SELECT_EXISTS_QUERY: &str = "selectExistsQuery";
pub const DELETE_QUERY: &str = "deleteQuery";
pub const DELETE_METHOD: &str = "Delete";
pub const UPDATE_METHOD: &str = "genUpdate";

/// Writes the unique queries for the primary key
pub struct QueryWriterPrimaryKey<'a> {
    pub table_meta_data: TableMetaData<'a>,
}

// Static queries
impl<'a> QueryWriterPrimaryKey<'a> {
    pub(crate) fn write_static_queries(mut self) {
        let static_queries = vec![
            self.static_select_query(),
            self.static_select_exists_query(),
            self.static_delete_query(),
        ];

        write_static_queries(self.table_meta_data.line_writer, static_queries);
    }

    fn static_select_query(&mut self) -> WriteResult {
        (
            SELECT_QUERY.to_string(),
            format!(
                "select * from {}{}",
                self.table_meta_data.table_name,
                self.table_meta_data.where_clause()
            ),
        )
    }

    fn static_select_exists_query(&mut self) -> WriteResult {
        (
            SELECT_EXISTS_QUERY.to_string(),
            format!(
                "select exists(select 1 from {}{})",
                self.table_meta_data.table_name,
                self.table_meta_data.where_clause()
            ),
        )
    }

    fn static_delete_query(&mut self) -> WriteResult {
        (
            DELETE_QUERY.to_string(),
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
        self.write_select_exists_query();
        self.write_delete_query();
        self.write_updatable_columns();
        self.write_update_column();

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

            Logging.log(Self.{query}, statementArguments: arguments)

            let statement = try db.cachedStatement(sql: Self.{query})

            {SET_ARGUMENTS}

            return try {}.fetchOne(statement)
        }}
        ",
            self.table_meta_data.struct_name,
            values,
            self.table_meta_data.struct_name,
            query = SELECT_QUERY,
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

    fn write_select_exists_query(&mut self) {
        let values = encode_swift_properties(&self.table_meta_data.primary_keys());

        self.table_meta_data
            .line_writer
            .add_comment("Checks if a row exists");
        self.table_meta_data.line_writer.add_with_modifier(format!(
            "func genSelectExists(db: Database) throws -> Bool {{
            let arguments: StatementArguments = try [
                {values}
            ]

            Logging.log(Self.{SELECT_EXISTS_QUERY}, statementArguments: arguments)

            let statement = try db.cachedStatement(sql: Self.{SELECT_EXISTS_QUERY})

            {SET_ARGUMENTS}

            // This always returns a row
            return try Bool.fetchOne(statement)!
        }}
        "
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
            DELETE_METHOD,
            &[],
            &values.iter().collect::<Vec<_>>(),
            &format!("Self.{DELETE_QUERY}"),
            true,
        )
    }

    fn write_update_column(&mut self) {
        let switch = self
            .table_meta_data
            .swift_properties
            .iter()
            .map(|s| {
                format!(
                    "case .{p}(let val): try genUpdate{}(db: db, {p}: val, assertOneRowAffected: assertOneRowAffected)",
                    some_kind_of_uppercase_first_letter(&s.swift_property_name),
                    p = s.swift_property_name,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        self
            .table_meta_data
            .line_writer
            .add_with_modifier(format!("
            func {UPDATE_METHOD}(db: Database, column: UpdatableColumnWithValue, assertOneRowAffected: Bool = true) throws {{
                switch column {{
                    {switch}
                }}
            }}
            "));
    }

    fn write_updatable_columns(&mut self) {
        // Write the updatable columns
        let updatable_columns = self.table_meta_data.swift_properties.to_vec();

        // The ref makes it easier to call other functions
        let updatable_columns = updatable_columns.iter().collect::<Vec<_>>();
        let update = "update";
        let primary_keys = self
            .table_meta_data
            .primary_keys()
            .into_iter()
            .cloned()
            .map(|mut s| {
                s.refers_to_self = true;

                s
            })
            .collect::<Vec<_>>();

        for property in &updatable_columns {
            let mut values = primary_keys.clone();

            values.insert(0, <&SwiftProperty>::clone(property).clone());

            let query_name = format!(
                "{}.UpdatableColumn.update{}Query",
                self.table_meta_data.struct_name,
                some_kind_of_uppercase_first_letter(&property.swift_property_name),
            );

            self.execute_update_statement(
                &format!(
                    "{}{}",
                    some_kind_of_uppercase_first_letter(update),
                    some_kind_of_uppercase_first_letter(&property.swift_property_name)
                ),
                &[property],
                &values.iter().collect::<Vec<_>>(),
                &query_name,
                true,
            );
        }

        // Writes the dynamic updates
        let update_query = format!("update {} set ", self.table_meta_data.table_name);
        let pk = format!(
            "where {}",
            self.table_meta_data.primary_key_name_columns_separated()
        );
        let updatable_columns = self.table_meta_data.swift_properties.to_vec();
        let switched = updatable_columns
            .iter()
            .map(|u| {
                let mut for_encoding = u.clone();

                // This is the name that is used for later
                for_encoding.swift_property_name = "value".to_string();

                let encoded = encode_swift_properties(&[&for_encoding]);

                format!(
                    "case let .{name}(value):
                if !arguments.isEmpty {{
                    updateQuery += \", \"
                }}

                arguments += [{}]

                updateQuery += \"{name} = ?\"\
            ",
                    encoded,
                    name = u.swift_property_name
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let pk_encoded = primary_keys
            .iter()
            .map(|p| format!("arguments += [{}]", encode_swift_properties(&[p])))
            .collect::<Vec<_>>()
            .join("\n");

        self
            .table_meta_data
            .line_writer
            .add_with_modifier(format!("
            func genUpdateDynamic(db: Database, columns: [{}.UpdatableColumnWithValue], assertOneRowAffected: Bool = true, assertAtLeastOneUpdate: Bool = true) throws {{
                assert(!assertAtLeastOneUpdate || !columns.isEmpty)

                // Check for duplicates
                assert(Set(columns.map {{ $0.columnName }}).count == columns.count)

                if columns.isEmpty {{
                    return
                }}

                let pkQuery = \"{}\"
                var updateQuery = \"{}\"
                var arguments = StatementArguments()

                for column in columns {{
                    switch column {{
                        {}
                    }}
                }}

                {}

                let finalQuery = updateQuery + \" \" + pkQuery

                Logging.log(finalQuery, statementArguments: arguments)

                let statement = try db.cachedStatement(sql: finalQuery)

                {SET_ARGUMENTS}

                try statement.execute()

                if assertOneRowAffected {{
                    assert(db.changesCount == 1)
                }}
            }}
        ",
                                       self.table_meta_data.struct_name,
                                       pk,
                                       update_query,
                                       switched,
                                       pk_encoded
            ))
    }
}
