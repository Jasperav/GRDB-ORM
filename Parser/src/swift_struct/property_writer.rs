use crate::configuration::Config;
use crate::swift_property::{swift_property_and_type, SwiftProperty};
use crate::table_meta_data::TableMetaData;

/// Writes the properties to the LineWriter
pub struct PropertyWriter<'a> {
    pub table_meta_data: &'a mut TableMetaData<'a>,
    pub location: Location,
    pub config: &'a Config,
}

impl<'a> PropertyWriter<'a> {
    pub fn write(self) {
        self.table_meta_data
            .line_writer
            .add_comment("Mapped columns to properties");

        let properties: Vec<SwiftProperty> = match self.location {
            Location::MainStruct => self.table_meta_data.swift_properties.to_vec(),
            Location::PrimaryKeyStruct => self
                .table_meta_data
                .primary_keys()
                .iter()
                .map(|m| (*m).clone())
                .collect(),
        };

        for property in properties {
            let swift_property = swift_property_and_type(&property);

            self.table_meta_data.line_writer.add_with_modifier(format!(
                "{} {}",
                self.config.immutability(property.column.part_of_pk),
                swift_property
            ));
        }

        self.table_meta_data.line_writer.new_line();
    }
}

pub enum Location {
    MainStruct,
    PrimaryKeyStruct,
}
