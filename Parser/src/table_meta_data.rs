use crate::line_writer::LineWriter;
use crate::swift_property::SwiftProperty;

/// Holds information about the current processing table
pub struct TableMetaData<'a> {
    pub line_writer: &'a mut LineWriter,
    pub swift_properties: &'a Vec<SwiftProperty>,
    pub struct_name: &'a str,
    pub table_name: &'a str,
    pub primary_key_struct_name: &'a str,
}

impl<'a> TableMetaData<'a> {
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
