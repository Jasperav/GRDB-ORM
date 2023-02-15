use std::collections::HashSet;
use crate::configuration::Config;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use inflector::Inflector;
use sqlite_parser::{Column, Metadata, OnUpdateAndDelete, Type};
use std::fs::File;
use std::path::PathBuf;
use crate::primary_keys;

pub struct AndroidWriter<'a> {
    pub metadata: &'a Metadata,
    pub config: &'a Config,
}

struct TypeConverter {
    name: String,
    to_write: String,
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

        let imports = self.config.room.imports.iter().map(|a| format!("import {a}")).collect::<Vec<_>>().join("\n");
        let mappers = self.generate_type_converters(&entity, &imports);
        let entities = self.generate_tables(&entity, &imports);
        let daos = self.generate_daos(&entity, &imports);

        self.generate_database(&entity, &mappers, &imports, &entities, &daos);
    }

    fn generate_daos(&self, path: &PathBuf, imports: &str,) -> Vec<(String, String)> {
        let mut daos = vec![];

        for table in self.metadata.tables.values() {
            let table_name = &table.table_name;
            let type_name = self.config.create_type_name(&table_name);
            let dao = format!("{type_name}Dao");
            let path = path.join(format!("{dao}.kt"));
            let mut content = vec![
                format!("
                package entity
                import androidx.room.*
import java.util.*
{imports}

                @Dao
                interface {dao} {{
                @Delete
                suspend fun delete(entity: {type_name})
                @Insert
                suspend fun insert(entity: {type_name})
                @Update
                suspend fun update(entity: {type_name}): Int
                @Query(\"SELECT * FROM {table_name}\")
                suspend fun selectAll(): Array<{type_name}>
                "),
            ];

            for column in &table.columns {
                let raw = format!("update {} set {} = :value", table.table_name, column.name);
                let update_all_query = format!("@Query(\"{raw}\")");
                let update_method = column.name.to_upper_camel_case();
                let ty = self.kotlin_type(column);

                content.push(format!("{update_all_query}
                    suspend fun updateAll{update_method}(value: {ty})
                "));

                let mut arguments_query = vec![];
                let mut arguments_method = vec![];

                for pk in primary_keys(&table) {
                    arguments_query.push(format!("{p} = :{p}", p = pk.name));
                    arguments_method.push(format!("{}: {}", pk.name, self.kotlin_type(pk)));
                }

                let joined = arguments_query.join(" and ");
                let update_query = format!("@Query(\"{raw} where {joined}\")");
                let update_methods = arguments_method.join(", ");

                content.push(format!("{update_query}
                    suspend fun update{update_method}(value: {ty}, {update_methods})
                "));
            }

            content.push("}".to_string());

            std::fs::write(path, content.join("\n")).unwrap();

            // Don't use prefix/suffix here, it looks better when calling the dao functions
            daos.push((table_name.to_string().to_lower_camel_case() + &"Dao", dao));
        }

        daos
    }

    fn generate_type_converters(&self, path: &PathBuf, imports: &str) -> String {
        if self.config.custom_mapping.is_empty() {
            return "".to_string()
        }

        let converters_path = path.join("GeneratedTypeConvertors.kt");
        let mut mappers = vec![];

        for mapping in &self.config.custom_mapping {
            if mapping.the_type == "Bool" || mapping.the_type == "Int64" {
                continue;
            }
                let kotlin_type = self.convert_swift_type_to_kotlin_type(&mapping.the_type);

            if self.config.room.skip_type_converters.contains(&kotlin_type) {
                continue;
            }

            let (parse_from, parse_to, ty) = if self.config.room.convert_with_gson_type_converters.contains(&mapping.the_type) {
                (format!("gson.fromJson(value, {kotlin_type}::class.java)"), "gson.toJson(value)".to_string(), "String")
            } else {
                if mapping.the_type == "UUID" {
                    (format!("UUID.fromString(value)"), "value.toString()".to_string(), "String")
                } else {
                    (format!("{kotlin_type}.parseFrom(value)"), "value.toByteArray()".to_string(), "ByteArray")
                }
            };

                let name = format!("Converter{}", kotlin_type);

                mappers.push(TypeConverter {
                    name: name.to_string(),
                    to_write: format!("class {name} {{
    @TypeConverter
    fun from(value: {ty}): {kotlin_type} {{
        return {parse_from}
    }}

    @TypeConverter
    fun to(value: {kotlin_type}): {ty} {{
        return {parse_to}
    }}
}}"),
                });
        }

        let to_write = mappers.iter().map(|m| m.to_write.to_string()).collect::<Vec<_>>().join("\n");
        let imports = format!("package entity\nimport androidx.room.TypeConverter\nimport java.util.*\n{imports}\nval gson = com.google.gson.Gson()\n{to_write}");

        std::fs::write(converters_path, imports).unwrap();

        let mapped = mappers.into_iter().map(|m| format!("{}::class", m.name)).collect::<Vec<_>>().join(", ");

        format!("@TypeConverters({mapped})")
    }

    fn generate_database(
        &self,
        path: &PathBuf,
        converters: &str,
        imports: &str,
        entities: &[String],
        daos: &[(String, String)],
    ) {
        let db = path.join("GeneratedDatabase.kt");

        File::create(&db).unwrap();

        let entities = entities
            .iter()
            .map(|t| {
                format!(
                    "{t}::class"
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        let daos = daos.iter().map(|(dao_method_name, dao)| {
            format!("abstract fun {dao_method_name}(): {dao}")
        })
            .collect::<Vec<_>>()
            .join("\n");
        let contents = format!(
            "
package entity

import androidx.room.Database
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
{imports}

        @Database(entities = [\n{entities}\n], version = 1)
{converters}
            abstract class GeneratedDatabase : RoomDatabase() {{
                {daos}
            }}
        "
        );

        std::fs::write(db, contents).unwrap();
    }

    fn generate_tables(&self, path: &PathBuf, imports: &str) -> Vec<String> {
        let mut entities = vec![];

        for table in self.metadata.tables.values() {
            let class_name = self.config.create_type_name(&table.table_name.to_upper_camel_case());

            entities.push(class_name.clone());

            let path = path.join(class_name.clone() + ".kt");

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
                "import androidx.room.ColumnInfo".to_string(),
                "import java.util.*".to_string(),
                imports.to_string(),
            ];
            let mut columns = vec![];
            let mut primary_keys = vec![];

            for column in &table.columns {
                let camel_case = column.name.to_camel_case();

                if column.part_of_pk {
                    primary_keys.push(format!("\"{}\"", camel_case.clone()));
                }

                let annotation = format!("@ColumnInfo(typeAffinity = ColumnInfo.{})\n", match column.the_type {
                    Type::Text => "TEXT",
                    Type::Integer => "INTEGER",
                    Type::String => "STRING",
                    Type::Real => "REAL",
                    Type::Blob => "BLOB"
                });

                columns.push(format!("{annotation}val {}: {}", camel_case, self.kotlin_type(&column)));
            }

            let primary_keys = primary_keys.join(", ");
            let mut unique_indexes = HashSet::<String>::default();

            for unique_index in &self.config.room.unique_indexes {
                let split = unique_index.split('.').collect::<Vec<_>>();

                assert_eq!(2, split.len());

                if split[0] != table.table_name {
                    continue;
                }

                assert!(unique_indexes.insert(split[1].to_string()));
            }

            let indices = if table.indexes.is_empty() && unique_indexes.is_empty() {
                "".to_string()
            } else {
                let mut indexes = vec![];

                for index in &table.indexes {
                    let index_formatted = index
                        .columns
                        .iter()
                        .map(|i| format!("\"{}\"", i.name))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let replace_unique_index = index.columns.len() == 1 && unique_indexes.remove(&index.columns[0].name);

                    let suffix = if index.unique || replace_unique_index {
                        ", unique = true"
                    } else {
                        ""
                    };

                    indexes.push(format!("Index(value = [{index_formatted}]{suffix})"));
                }

                for unique_index in unique_indexes {
                    indexes.push(format!("Index(value = [\"{unique_index}\"], unique = true)"));
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
                        "entity = {}::class,\n",
                        self.config.create_type_name(&foreign_key.table)
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

        entities
    }

    fn kotlin_type(&self, column: &Column) -> String {
        let mut value = None;

        for mapping in &self.config.custom_mapping {
            if mapping
                .regexes
                .iter()
                .any(|regex| regex.is_match(&column.name))
            {
                value = Some(mapping.the_type.to_string());

                break;
            }
        }

        let new_value = if let Some(val) = value {
             let result = match val.as_str() {
                "Bool" => "Boolean".to_string(),
                "Int64" => "Long".to_string(),
                 // Special value
                 "meta" => "ByteArray".to_string(),
                _ => {
                    self.convert_swift_type_to_kotlin_type(&val)
                }
            };

            Some(result)
        } else {
            if column.name == "meta" {
                // Special value
                Some("ByteArray".to_string())
            } else {
                None
            }
        };

        let mut result = match new_value {
            None => match column.the_type {
                Type::Text | Type::String => "String".to_string(),
                Type::Integer => "Int".to_string(),
                Type::Real => "Double".to_string(),
                Type::Blob => panic!("Should already been mapped: {:#?}, tables: {:#?}", column, self.metadata.tables),
            }
            Some(val) => val
        };

        if column.nullable {
            result += "?"
        }

        result
    }

    fn convert_swift_type_to_kotlin_type(&self, swift_type: &str) -> String {
        let split = swift_type.split("_").collect::<Vec<_>>();

        if split.len() <= 1 {
            if swift_type == "Bool" {
                "Boolean".to_string()
            } else {
                swift_type.to_string()
            }
        } else {
            // This is a Swift API Protobuf type, like Data_AppRole, convert it to kotlin
            split.last().unwrap().to_string()
        }
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
