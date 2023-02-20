use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use inflector::Inflector;
use sqlite_parser::{Table, Type};
use crate::android::android::AndroidWriter;
use crate::primary_keys;

impl<'a> AndroidWriter<'a> {
    pub(crate) fn generate_tables(&self, path: &PathBuf, imports: &str) -> Vec<String> {
        let mut entities = vec![];

        for table in self.metadata.tables.values() {
            let class_name = self.config.create_type_name(&table.table_name.to_upper_camel_case());

            entities.push(class_name.clone());

            let path = path.join(class_name.clone() + ".kt");

            File::create(&path).unwrap();

            let mut contents = vec![
                "package entity".to_string(),
                "import androidx.room.ForeignKey".to_string(),
                "import androidx.room.ForeignKey.Companion.NO_ACTION".to_string(),
                "import androidx.room.ForeignKey.Companion.RESTRICT".to_string(),
                "import androidx.room.ForeignKey.Companion.SET_NULL".to_string(),
                "import androidx.room.ForeignKey.Companion.SET_DEFAULT".to_string(),
                "import androidx.room.ForeignKey.Companion.CASCADE".to_string(),
                "import androidx.room.ColumnInfo".to_string(),
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

                columns.push(format!("{annotation}var {}: {}", camel_case, self.kotlin_type(&column)));
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

            // Now add the updatable columns
            let updatable_columns = self.updatable_columns(table);
            let pk_class = self.generate_primary_keys(table);

            contents.push(format!("@Entity(\ntableName = \"{}\",\nprimaryKeys = [{}]{indices}\n{foreign_keys})\ndata class {class_name}(\n{}\n)\n{{ {updatable_columns}\n{pk_class} \n}}", table.table_name, primary_keys, columns.join(",\n")));

            std::fs::write(path, contents.join("\n")).unwrap();
        }

        entities
    }

    fn generate_primary_keys(&self, table: &Table) -> String {
        let mut pks = vec![];

        for pk in primary_keys(table) {
            let kotlin_ty = self.kotlin_type(pk);

            pks.push(format!("val {}: {kotlin_ty}", pk.name));
        }

        let pks = pks.join(",\n");

        format!("data class PrimaryKey(\n{pks}\n)")
    }

    fn updatable_columns(&self, table: &Table) -> String {
        let mut columns_updatable_value = vec![];
        let mut columns_updatable = vec![];
        let mut switch = vec![];

        for column in &table.columns {
            let kotlin_ty = self.kotlin_type(column);
            let column_name = &column.name;
            let class_name = column_name.to_upper_camel_case();

            columns_updatable.push(format!("{}", class_name.to_shouty_snake_case()));
            columns_updatable_value.push(format!("data class {class_name}(val {column_name}: {kotlin_ty}): UpdatableColumnWithValue()"));
            switch.push(format!("is {class_name} -> entity.{column_name} = {column_name}"));
        }

        let columns_updatable_value = columns_updatable_value.join("\n");
        let columns_updatable = columns_updatable.join(",\n");
        let switch = switch.join("\n");
        let table_name = self.config.create_type_name(&table.table_name);

        format!("sealed class UpdatableColumnWithValue {{
        {columns_updatable_value}
        fun update(entity: {table_name}) {{
            when (this) {{
                {switch}
            }}
        }}
        }}
        enum class UpdatableColumn {{
        {columns_updatable}
        }}
        ")
    }
}