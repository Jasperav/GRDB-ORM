use std::path::PathBuf;

use sqlite_parser::Metadata;

use crate::configuration::Config;
use crate::query_writer::{QueryWriterMainStruct, QueryWriterPrimaryKey};
use crate::swift_property::{create_swift_properties, create_swift_type_name};
use crate::swift_struct::main_struct_to_pk::write_main_struct_to_pk;
use crate::swift_struct::property_writer::{Location, PropertyWriter};
use crate::table_meta_data::TableMetaData;

/// Starting point to write generated code from a table
/// This struct writes a struct for a given table and a primary key struct
pub struct TableWriter<'a> {
    pub tables: &'a Metadata,
    pub config: &'a Config,
    pub safe_output_dir: PathBuf,
}

impl<'a> TableWriter<'a> {
    // This is needed else a compile warning turns up but that is a false positive
    #[allow(unused_braces)]
    pub fn write(self) {
        println!("Preparing to process the tables");

        // Time to generate the structs
        for (table_name, table) in &self.tables.tables {
            println!("Processing table {}", table_name);

            // The lines to eventually write
            let mut line_writer = self.config.create_line_writer();

            line_writer.add_line("import GRDB\nimport Foundation".to_string());
            line_writer.new_line();

            // Create swift properties, taken the configuration into account
            let swift_properties = create_swift_properties(table, &self.config.custom_mapping);

            // The struct name to write
            let struct_name = create_swift_type_name(table_name, &self.config);

            // The primary key struct name to write
            let primary_key_struct_name =
                create_swift_type_name(&(table_name.clone() + "PrimaryKey"), &self.config);

            // Start the actual writing
            line_writer.add_comment("Mapped table to struct");
            line_writer.add_with_modifier(format!(
                "struct {}: FetchableRecord, PersistableRecord, Codable {{\n",
                struct_name
            ));

            // Pretty complicated macro, but ensures no duplicate code is written
            macro_rules! qw {
                () => {
                    TableMetaData {
                        line_writer: &mut line_writer,
                        swift_properties: &swift_properties,
                        struct_name: &struct_name,
                        table_name: &table_name,
                        primary_key_struct_name: &primary_key_struct_name
                    }
                };
                (MainStruct) => {
                    QueryWriterMainStruct::new(qw!())
                };
                (PrimaryKeyStruct) => {
                    QueryWriterPrimaryKey {
                        table_meta_data: qw!(),
                    }
                };
                (flow = $flow: ident, $tt: tt) => {
                    line_writer.new_line();
                    // Write the static queries
                    qw!($flow).write_static_queries();

                    // Next, write the properties
                    PropertyWriter {
                        table_meta_data: &mut qw!(),
                        location: Location::$flow,
                        config: self.config
                    }.write();

                    $tt

                    // Write the methods
                    qw!($flow).write_method();
                }
            }

            // Start of by writing the 'main struct'
            qw!(flow = MainStruct, {
                crate::swift_struct::initializer::write_initializer(&mut qw!())
            });

            // Now write the primary key struct
            line_writer.add_comment(
                "Write the primary key struct, useful for selecting or deleting a unique row",
            );
            line_writer.add_with_modifier(format!("struct {} {{\n", primary_key_struct_name));

            let pk = swift_properties
                .iter()
                .filter(|s| s.column.part_of_pk)
                .collect();

            qw!(flow = PrimaryKeyStruct, {
                crate::swift_struct::initializer::write_default_initializer(&mut line_writer, &pk)
            });

            // Write conversation from MainStruct to PrimaryKey
            write_main_struct_to_pk(&mut qw!());

            line_writer.write_to_file(&struct_name);
        }

        println!("Successfully parsed all tables");
    }
}
