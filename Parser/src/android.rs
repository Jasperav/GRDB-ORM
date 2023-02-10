use std::fs::File;
use heck::ToUpperCamelCase;
use inflector::Inflector;
use sqlite_parser::{Column, Metadata, Type};
use crate::configuration::Config;

pub struct AndroidWriter<'a> {
    pub metadata: &'a Metadata,
    pub config: &'a Config,
}

impl <'a> AndroidWriter<'a> {
    pub fn parse(&self) {
        if self.config.output_dir_android.exists() && self.config.output_dir_android.is_dir() {
            println!("Generating Android room objects...");
        } else {
            println!("Won't generate Android objects");
        }

        assert!(self.config.output_dir_android.ends_with("java"));

        // Don't use the generated suffix here, stupid packages...
        let _ = std::fs::remove_dir_all(&self.config.output_dir_android);

        // Create the folder to put the generated files in
        std::fs::create_dir_all(&self.config.output_dir_android).unwrap();

        self.generate_tables();
    }

    fn generate_tables(&self) {
        for table in self.metadata.tables.values() {
            let class_name = format!("{}{}{}", self.config.prefix_swift_structs, table.table_name.to_upper_camel_case(), self.config.suffix_swift_structs);
            let path = self.config.output_dir_android.join(class_name.clone() + ".kt");

            File::create(&path).unwrap();

            let mut contents = vec!["import androidx.room.*".to_string()];
            let mut columns = vec![];
            let mut primary_keys = vec![];

            for column in &table.columns {
                let camel_case = column.name.to_camel_case();

                if column.part_of_pk {
                    primary_keys.push(format!("\"{}\"", camel_case.clone()));
                }

                columns.push(format!("val {}: {}", camel_case, self.kotlin_type(&column)));
            }

            let primary_keys = primary_keys.join(", ");

            contents.push(format!("@Entity(tableName = \"{}\", primaryKeys = [{}])\ndata class {class_name}(\n{}\n)", table.table_name, primary_keys, columns.join(",\n")));

            std::fs::write(path, contents.join("\n")).unwrap();
        }
    }

    fn kotlin_type(&self, column: &Column) -> String {
        let mut result = match column.the_type {
            Type::Text | Type::String => "String",
            Type::Integer => {
                let mut value = None;

                for mapping in &self.config.custom_mapping {
                    if mapping
                        .regexes
                        .iter()
                        .any(|regex| regex.is_match(&column.name))
                    {
                        value = Some(if mapping.the_type == "Bool" {
                            "Boolean"
                        } else {
                            assert_eq!(mapping.the_type, "Int64");

                            "Long"
                        });
                    }
                }

                value.unwrap_or("Int")
            },
            Type::Real => "Double",
            Type::Blob => "java.sql.Blob",
        }.to_string();

        if column.nullable {
            result += "?"
        }

        result
    }
}
