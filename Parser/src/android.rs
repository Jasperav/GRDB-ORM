use crate::configuration::Config;
use heck::ToUpperCamelCase;
use inflector::Inflector;
use sqlite_parser::{Column, Metadata, OnUpdateAndDelete, Type};
use std::fs::File;
use std::path::PathBuf;

pub struct AndroidWriter<'a> {
    pub metadata: &'a Metadata,
    pub config: &'a Config,
}

impl<'a> AndroidWriter<'a> {
    pub fn parse(&self) {
        if self.config.output_dir_android.exists() && self.config.output_dir_android.is_dir() {
            println!("Generating Android room objects...");
        } else {
            println!("Won't generate Android objects");
        }

        assert!(self.config.output_dir_android.ends_with("java"));

        let entity = self.config.output_dir_android.join("entity");

        // Don't use the generated suffix here, stupid packages...
        let _ = std::fs::remove_dir_all(&entity);

        // Create the folder to put the generated files in
        std::fs::create_dir_all(&entity).unwrap();

        self.generate_tables(&entity);
        self.generate_database(&entity);
    }

    fn generate_database(&self, path: &PathBuf) {
        let db = path.join("GeneratedDatabase.kt");

        File::create(&db).unwrap();

        let entities = self.metadata.tables.iter().map(|t| format!("{}{}{}::class", self.config.prefix_swift_structs, t.0, self.config.suffix_swift_structs)).collect::<Vec<_>>().join(",\n");
        let contents = format!("
package entity

import androidx.room.Database
import androidx.room.RoomDatabase

        @Database(entities = [\n{entities}\n], version = 1)
            abstract class GeneratedDatabase : RoomDatabase() {{
            }}
        ");

        std::fs::write(db, contents).unwrap();
    }

    fn generate_tables(&self, path: &PathBuf) {
        for table in self.metadata.tables.values() {
            let class_name = format!(
                "{}{}{}",
                self.config.prefix_swift_structs,
                table.table_name.to_upper_camel_case(),
                self.config.suffix_swift_structs
            );
            let path = path
                .join(class_name.clone() + ".kt");

            File::create(&path).unwrap();

            let mut contents = vec![
                "package entity".to_string(),
                "import androidx.room.*".to_string(),
                "import androidx.room.ForeignKey".to_string(),
                "import androidx.room.ForeignKey.Companion.NO_ACTION".to_string(),
                "import androidx.room.ForeignKey.Companion.RESTRICT".to_string(),
                "import androidx.room.ForeignKey.Companion.SET_NULL".to_string(),
                "import androidx.room.ForeignKey.Companion.SET_DEFAULT".to_string(),
                "import androidx.room.ForeignKey.Companion.CASCADE".to_string(),
            ];
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
            let indices = if table.indexes.is_empty() {
                "".to_string()
            } else {
                let mut indexes = vec![];

                for index in &table.indexes {
                    let index = index
                        .columns
                        .iter()
                        .map(|i| format!("\"{}\"", i.name))
                        .collect::<Vec<_>>()
                        .join(", ");

                    indexes.push(format!("Index(value = [{index}])"));
                }

                format!(", indices = [{}]", indexes.join(", "))
            };
            let foreign_keys = if table.foreign_keys.is_empty() {
                "".to_string()
            } else {
                let mut foreign_keys = vec![];

                for foreign_key in &table.foreign_keys {
                    // Currently only 1 column is supported
                    assert_eq!(1, foreign_key.to_column.len());
                    assert_eq!(foreign_key.from_column.len(), foreign_key.to_column.len());

                    let mut start = "ForeignKey(\n".to_string();

                    start += &format!(
                        "entity = {}{}{}::class,\n",
                        self.config.prefix_swift_structs,
                        foreign_key.table,
                        self.config.suffix_swift_structs
                    );
                    start += &format!(
                        "childColumns = [\"{}\"],\n",
                        foreign_key.from_column[0].name
                    );
                    start += &format!("parentColumns = [\"{}\"],\n", foreign_key.to_column[0].name);
                    start += &format!(
                        "onDelete = {},\n",
                        self.convert_to_foreign_key(foreign_key.on_delete)
                    );
                    start += &format!(
                        "onUpdate = {},\n)",
                        self.convert_to_foreign_key(foreign_key.on_update)
                    );

                    foreign_keys.push(start);
                }

                format!(", foreignKeys = [{}]", foreign_keys.join(", "))
            };

            contents.push(format!("@Entity(\ntableName = \"{}\",\nprimaryKeys = [{}]{indices}\n{foreign_keys})\ndata class {class_name}(\n{}\n)", table.table_name, primary_keys, columns.join(",\n")));

            std::fs::write(path, contents.join("\n")).unwrap();
        }
    }

    fn kotlin_type(&self, column: &Column) -> String {
        // TODO: support UUID with converters
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
            }
            Type::Real => "Double",
            Type::Blob => "java.sql.Blob",
        }
        .to_string();

        if column.nullable {
            result += "?"
        }

        result
    }

    fn convert_to_foreign_key(&self, on_update_and_delete: OnUpdateAndDelete) -> &'static str {
        match on_update_and_delete {
            OnUpdateAndDelete::NoAction => "NO_ACTION",
            OnUpdateAndDelete::Restrict => "RESTRICT",
            OnUpdateAndDelete::SetNull => "SET_NULL",
            OnUpdateAndDelete::SetDefault => "SET_DEFAULT",
            OnUpdateAndDelete::Cascade => "CASCADE",
        }
    }
}
