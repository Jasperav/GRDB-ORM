use crate::line_writer::{parameter_types_separated_colon, LineWriter, StaticInstance, WriteRead};
use crate::swift_property::{swift_properties_to_sqlite_database_values, SwiftProperty};

/// Holds information about the current processing table
pub struct TableMetaData<'a> {
    pub line_writer: &'a mut LineWriter,
    pub swift_properties: &'a Vec<SwiftProperty>,
    pub struct_name: &'a str,
    pub table_name: &'a str,
    pub primary_key_struct_name: &'a str,
}

impl<'a> TableMetaData<'a> {
    pub fn write_update_with_wrapper(
        &mut self,
        fn_name: &str,
        parameters: &[&SwiftProperty],
        values: &[&SwiftProperty],
        sql: &str,
        is_auto_generated: bool,
    ) {
        self.line_writer.add_with_modifier(format!(
            "func {}(db: Database{}) throws {{
            let arguments: StatementArguments = try [
                {}
            ]

            let statement = try db.cachedUpdateStatement(sql: {})

            statement.setUncheckedArguments(arguments)

            try statement.execute()

            assert(db.changesCount == 1)
        }}
        ",
            if is_auto_generated {
                format!("gen{}", fn_name)
            } else {
                fn_name.to_string()
            },
            parameter_types_separated_colon(parameters),
            swift_properties_to_sqlite_database_values(values),
            sql
        ));

        self.line_writer.add_wrapper_pool(
            StaticInstance::Instance,
            fn_name,
            WriteRead::Write,
            is_auto_generated,
            &parameters.to_vec(),
        );
    }
    fn keys(&self, part_of_pk: bool) -> Vec<&SwiftProperty> {
        self.swift_properties
            .iter()
            .filter(|c| c.column.part_of_pk == part_of_pk)
            .collect()
    }

    pub fn non_primary_keys(&self) -> Vec<&SwiftProperty> {
        self.keys(false)
    }

    pub fn primary_keys(&self) -> Vec<&SwiftProperty> {
        self.keys(true)
    }

    pub fn where_clause(&self) -> String {
        format!(" where {}", self.primary_key_name_columns_separated())
    }

    pub fn primary_key_name_columns_separated(&self) -> String {
        self.primary_keys()
            .iter()
            .map(|pk| format!("{} = ?", pk.column.name))
            .collect::<Vec<_>>()
            .join(" and ")
    }
}
