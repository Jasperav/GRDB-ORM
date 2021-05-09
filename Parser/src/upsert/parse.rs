use crate::parse::{test_query, Parser};
use crate::query_writer::QueryWriterMainStruct;
use crate::swift_property::{create_swift_properties, create_swift_type_name};
use crate::table_meta_data::TableMetaData;

/// Parses the upserts
impl<'a> Parser<'a> {
    pub fn parse_upserts(mut self) {
        if self.config.upserts.is_empty() {
            println!("No upserts found");

            return;
        }

        // Write the generic imports
        self.write_imports("upserts");

        // Process each upsert
        for upsert in &self.config.upserts {
            // Not logical if no updates are present
            assert!(!upsert.columns_to_update.is_empty());

            let swift_properties = create_swift_properties(
                self.tables.table(&upsert.table).unwrap(),
                &self.config.custom_mapping,
            );

            let struct_name = create_swift_type_name(&upsert.table, &self.config);

            macro_rules! tmd {
                () => {
                    TableMetaData {
                        line_writer: &mut self.line_writer,
                        swift_properties: &swift_properties,
                        struct_name: &struct_name,
                        table_name: &upsert.table,
                        primary_key_struct_name: "",
                    }
                };
            }
            // Bit ugly way to get the insert query
            let insert_query = QueryWriterMainStruct::new(tmd!())
                .static_unique_insert_query()
                .1;
            let mut tmd = tmd!();

            // Create a comma separated string of primary keys, used for ON CONFLICT clause in the query
            let pk_comma = tmd
                .primary_keys()
                .into_iter()
                .map(|t| t.column.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            // This is how you write an upsert query
            let update = upsert
                .columns_to_update
                .iter()
                .map(|c| {
                    assert!(tmd
                        .swift_properties
                        .iter()
                        .any(|s| s.column.name.to_lowercase() == c.to_lowercase()));

                    format!("{column}=excluded.{column}", column = c)
                })
                .collect::<Vec<_>>()
                .join(", ");

            // This is the actual query
            let query = format!(
                "{} on conflict ({}) do update set {}",
                insert_query, pk_comma, update
            );

            // Don't actually test the query because the table can have mandatory foreign keys

            let values = tmd.swift_properties.clone();

            tmd.line_writer
                .add_line(format!("extension {} {{", &struct_name));
            tmd.write_update_with_wrapper(
                &upsert.func_name,
                &[],
                &values.iter().collect::<Vec<_>>(),
                &format!("\"{}\"", query),
                false,
            );
            tmd.line_writer.add_closing_brackets();
        }

        self.write("Upserts");
    }
}
