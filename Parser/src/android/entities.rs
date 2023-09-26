use crate::android::kotlin::AndroidWriter;
use crate::android::{generate_kotlin_package, SUPPRESS_WILDCARD_IMPORTS};
use crate::primary_keys;
use heck::ToUpperCamelCase;
use inflector::Inflector;
use sqlite_parser::{Column, Table, Type};
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;

impl<'a> AndroidWriter<'a> {
    pub(crate) fn generate_tables(&self, path: &Path, imports: &str) -> Vec<String> {
        let mut entities = vec![];

        for table in self.metadata.tables.values() {
            let class_name = self
                .config
                .create_type_name(&table.table_name.to_upper_camel_case());

            entities.push(class_name.clone());

            let package = generate_kotlin_package(path);
            let path = path.join(class_name.clone() + ".kt");

            File::create(&path).unwrap();

            let mut contents = vec![
                SUPPRESS_WILDCARD_IMPORTS.to_string(),
                package,
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

                let annotation = format!(
                    "@ColumnInfo(typeAffinity = ColumnInfo.{})\n",
                    match column.the_type {
                        Type::Text => "TEXT",
                        Type::Integer => "INTEGER",
                        Type::String => "STRING",
                        Type::Real => "REAL",
                        Type::Blob => "BLOB",
                    }
                );

                columns.push(format!(
                    "{annotation}var {}: {}",
                    camel_case,
                    self.kotlin_type(column)
                ));
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
                    let replace_unique_index =
                        index.columns.len() == 1 && unique_indexes.remove(&index.columns[0].name);

                    let suffix = if index.unique || replace_unique_index {
                        ", unique = true"
                    } else {
                        ""
                    };

                    indexes.push(format!("Index(value = [{index_formatted}]{suffix})"));
                }

                for unique_index in unique_indexes {
                    indexes.push(format!(
                        "Index(value = [\"{unique_index}\"], unique = true)"
                    ));
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
            let upserts = self.generate_upsert(table);
            let custom_code = self.interfaces_for_ty(&class_name);

            contents.push(format!(
                "\
            @Entity(
                tableName = \"{}\",
                primaryKeys = [{}]{indices}
                {foreign_keys})
            data class {class_name}(
                {}
            ) {custom_code}
                {upserts}
                {updatable_columns}
                {pk_class}
            }}",
                table.table_name,
                primary_keys,
                columns.join(",\n")
            ));

            std::fs::write(path, contents.join("\n")).unwrap();
        }

        entities
    }

    fn generate_upsert(&self, table: &Table) -> String {
        let mut upsert_dyn = vec![];
        let mut insert_query = vec![];
        let mut values = vec![];
        let mut pk_values = vec![];
        let mut static_queries = vec![];
        let mut upserts = vec![];

        for pk in primary_keys(table) {
            pk_values.push(pk.name.to_string());
        }

        for column in &table.columns {
            let name = column.name.clone();
            let upper_camel_cased = name.to_upper_camel_case();
            let ty = self.kotlin_type(column);

            insert_query.push(column.name.to_string());
            upsert_dyn.push(format!(
                "UpdatableColumn.{} -> {{
                if (processedAtLeastOneColumns) {{
                    query += \", \"
                }}
                query += \"{name}=excluded.{name}\"\
            }}
            ",
                upper_camel_cased
            ));
            values.push("?");

            let bind_single = self.bind_single(column, &mut vec![], 1, true);
            let delete_query = format!("delete from {} where {name} = ?", table.table_name);
            let update_query = format!("update {} set {name} = ?", table.table_name);

            static_queries.push(format!(
                "fun deleteBy{upper_camel_cased}(database: GeneratedDatabase, {name}: {ty}) {{
                val stmt = database.compileCached(\"{delete_query}\", {name})
                assert(database.inTransaction())

                {bind_single}

                stmt.execute()
            }}
            fun update{upper_camel_cased}AllRows(database: GeneratedDatabase, {name}: {ty}) {{
                val stmt = database.compileCached(\"{update_query}\", {name})
                assert(database.inTransaction())

                {bind_single}

                stmt.execute()
            }}"
            ));
            upserts.push(format!(
                "fun upsert{upper_camel_cased}(database: GeneratedDatabase) {{
                upsertDynamic(database, listOf(UpdatableColumn.{upper_camel_cased}))
            }}"
            ));
        }

        // TODO: if this doesn't work with updating live data, just delete it and use the DAO
        for query in &self.config.dynamic_queries {
            if (query.extension.to_lowercase() != table.table_name.to_lowercase()
                && query.extension.to_lowercase()
                    != self
                        .config
                        .create_type_name(&table.table_name)
                        .to_lowercase())
                || query.query.trim().starts_with("select")
            {
                continue;
            }

            let mut parameters = vec![];
            let mut binding = vec![];
            let mut custom_names = vec![];

            for (table, column, arg) in &query.parameter_types {
                let mut kotlin_type = self.convert_parameter_type_to_kotlin_type(table, column);

                let column = if table == "Int" {
                    let c = Column {
                        id: 0,
                        name: arg.to_string(),
                        the_type: Type::Integer,
                        nullable: false,
                        part_of_pk: false,
                    };

                    // Hack
                    if self.kotlin_type(&c) == "Boolean" {
                        kotlin_type = "Boolean".to_string();
                    }

                    c
                } else {
                    self.kotlin_column_from_table_column(table, column)
                };

                parameters.push(format!("{}: {}", arg, kotlin_type));
                binding.push(column);
                custom_names.push(arg.to_string())
            }

            let b = self.bind(&query.query, &binding, false, custom_names);

            let mut parameters = parameters.join(", ");

            if !parameters.is_empty() {
                parameters = ", ".to_string() + &parameters;
            }

            static_queries.push(format!(
                "fun {}(database: GeneratedDatabase{}) {{
                {b}
             }}",
                query.func_name, parameters
            ));
        }

        let pk_values = pk_values.join(", ");
        let insert_query = format!("\"insert into {}(", table.table_name)
            + &(insert_query.join(", ")
                + &((") VALUES (".to_string() + &values.join(", ")) + ")\""));
        let insert_or_ignore_query = insert_query.replace("insert into", "insert or ignore into");
        let replace_query = insert_query.replace("insert into", "replace into");
        let upsert = upsert_dyn.join("\n");
        let bind = self.bind("query", &table.columns, false, vec![]);
        let static_queries = static_queries.join("\n\n");
        let upserts = upserts.join("\n\n");

        format!("        fun upsertDynamic(database: GeneratedDatabase, columns: List<UpdatableColumn>) {{
            assert(database.inTransaction())
            if (columns.isEmpty()) {{
                return
            }}

            val insertQuery = {insert_query}
            var query = insertQuery + \"on conflict ({pk_values}) do update set \"
            var processedAtLeastOneColumns = false

            for (column in columns) {{
                when (column) {{
                    {upsert}
                }}

                processedAtLeastOneColumns = true
            }}

            {bind}

        }}
        fun insertOrIgnore(database: GeneratedDatabase) {{
            val query = {insert_or_ignore_query}
            assert(database.inTransaction())

            {bind}
        }}
        fun insert(database: GeneratedDatabase) {{
            val query = {insert_query}
            assert(database.inTransaction())

            {bind}
        }}
        fun replace(database: GeneratedDatabase) {{
            val query = {replace_query}
            assert(database.inTransaction())

            {bind}
        }}

        {upserts}

        companion object {{
            {static_queries}
        }}
        ")
    }

    fn bind(
        &self,
        query: &str,
        columns: &Vec<Column>,
        update_or_delete: bool,
        mut custom_names: Vec<String>,
    ) -> String {
        let mut bindings = vec![];
        let mut names = vec![];

        for column in columns {
            if self.config.android_verbose_sql_logging {
                names.push(custom_names.get(0).unwrap_or(&column.name).to_owned());
            }

            let binding = self.bind_single(column, &mut custom_names, bindings.len() + 1, true);

            bindings.push(binding);
        }

        let bindings = bindings.join("\n");
        let update_delete = if update_or_delete {
            "executeUpdateDelete"
        } else {
            "execute"
        };
        let query = if query.contains(' ') {
            format!("\"{query}\"")
        } else {
            query.to_string()
        };
        let mut joined = names.join(", ");

        if !joined.is_empty() {
            joined = ", ".to_string() + &joined;
        }

        format!(
            "val stmt = database.compileCached({query}{joined})
            {bindings}

        val ex = stmt.{update_delete}()
        "
        )
    }

    fn bind_single(
        &self,
        column: &Column,
        custom_names: &mut Vec<String>,
        index: usize,
        use_index_as_binding: bool,
    ) -> String {
        assert!(index > 0);

        let kotlin_ty = self.kotlin_type(column);
        let without_opt = kotlin_ty.replace('?', "");
        let name = if custom_names.is_empty() {
            column.name.to_string()
        } else {
            custom_names.remove(0)
        };
        let variable_name = format!("val{index}");

        let (default_binding, ty) = if without_opt == "UUID" {
            (format!("ConverterUUID().to({name})"), "String")
        } else if kotlin_ty == "Boolean" {
            (format!("if ({name}) {{ 1L }} else {{ 0L }}"), "Long")
        } else if without_opt == "Long" {
            (name, "Long")
        } else if without_opt == "Int" {
            (format!("{name}?.toLong()"), "Long")
        } else if without_opt == "Double" {
            (name, "Double")
        } else if column.the_type == Type::Blob {
            if without_opt == "ByteArray" {
                (name, "Blob")
            } else {
                (
                    format!(
                        "Converter{}().to({name})",
                        without_opt.to_upper_camel_case()
                    ),
                    "Blob",
                )
            }
        } else if column.the_type == Type::Text {
            if self
                .config
                .room
                .convert_with_gson_type_converters
                .contains(&kotlin_ty)
            {
                (format!("Converter{kotlin_ty}().to({name})"), "String")
            } else {
                (format!("{name}?.let {{ it.toString() }}"), "String")
            }
        } else {
            panic!()
        };

        let (index, post_binding) = if use_index_as_binding {
            (format!("{index}"), "")
        } else {
            ("index".to_string(), "index += 1")
        };

        format!(
            "val {variable_name} = {default_binding}
            if ({variable_name} == null) {{
                stmt.bindNull({index});
            }} else {{
                stmt.bind{ty}({index}, {variable_name});
            }}
            {post_binding}"
        )
    }

    fn generate_primary_keys(&self, table: &Table) -> String {
        let mut pks = vec![];
        let mut pk_in_query = vec![];
        let primary_keys = primary_keys(table);
        let mut delete_query = format!("delete from {} where ", table.table_name);
        let mut delete_bindings = vec![];
        let mut convert_to_pk = vec![];
        let mut names = vec![];

        for pk in &primary_keys {
            let kotlin_ty = self.kotlin_type(pk);

            pks.push(format!("val {}: {kotlin_ty}", pk.name));
            pk_in_query.push(format!("{} = ?", pk.name));
            delete_bindings.push(self.bind_single(
                pk,
                &mut vec![],
                delete_bindings.len() + 1,
                true,
            ));
            convert_to_pk.push(pk.name.clone());

            if self.config.android_verbose_sql_logging {
                names.push(pk.name.to_string());
            }
        }

        let pks_in_query = pk_in_query.join(" and ");
        let delete_bindings = delete_bindings.join("\n");
        let names = names.join(", ");

        delete_query += &pks_in_query;

        let delete_query = format!("fun delete(database: GeneratedDatabase, assertOneRowAffected: Boolean = true): Boolean {{
            val query = \"{delete_query}\"
            assert(database.inTransaction())
            val stmt = database.compileCached(query, {names})

            {delete_bindings}

            val changed = stmt.executeUpdateDelete()

            if (assertOneRowAffected && changed == 0) {{
                 assert(false)
            }}

            return changed == 1
        }}");

        let mut update_single_column = vec![];

        for column in &table.columns {
            let camel_cased = &column.name.to_upper_camel_case();
            let kotlin_ty = self.kotlin_type(column);
            let update_query = format!(
                "val query = \"update {} set {} = ? where {}\"",
                table.table_name, column.name, pks_in_query
            );

            let mut column_different_name = column.clone();

            column_different_name.name = "value".to_string();

            let mut columns = vec![column];

            columns.extend(primary_keys.clone());
            let owned = columns.into_iter().cloned().collect();
            let binded = self.bind("query", &owned, true, vec!["value".to_string()]);

            update_single_column.push(format!("fun update{camel_cased}(database: GeneratedDatabase, value: {kotlin_ty}, assertOneRowAffected: Boolean = true): Boolean {{
                {update_query}
                {binded}
                assert(database.inTransaction())

                if (assertOneRowAffected && ex == 0) {{
                    assert(false)
                }}

                return ex == 1
            }}"))
        }

        let update_dyn_query = self.update_dyn_query(table);
        let pks = pks.join(",\n");
        let update_single_columns = update_single_column.join("\n");
        let convert = format!(
            "fun primaryKey(): PrimaryKey{{
            return PrimaryKey({})
        }}",
            convert_to_pk.join(", ")
        );

        format!("data class PrimaryKey(
        {pks}
     ){{
        {update_single_columns}

        fun updateDynamic(database: GeneratedDatabase, values: List<UpdatableColumnWithValue>, assertOneRowAffected: Boolean = true): Boolean {{
            if (values.isEmpty()) {{
                return false
            }}

            {update_dyn_query}
        }}

        {delete_query}
     }}

     {convert}")
    }

    fn update_dyn_query(&self, table: &Table) -> String {
        let mut where_clause = vec![];
        let mut bindings_pk = vec![];
        let mut index = 1;

        for pk in primary_keys(table) {
            where_clause.push(format!("{} = ?", pk.name));

            bindings_pk.push(self.bind_single(pk, &mut vec![], index, false));

            index += 1
        }

        let bindings_pk = bindings_pk.join("\n");
        let where_clause = "where ".to_string() + &where_clause.join(" and ");
        let mut contents = vec![
            format!("val names = mutableListOf<String>()"),
            format!("val pkQuery = \"{where_clause}\""),
            format!("var updateQuery = \"update {} set \"", table.table_name),
            format!("var index = 1"),
            format!("val closures = mutableListOf<(androidx.sqlite.db.SupportSQLiteStatement) -> Unit>()
            for (column in values) {{
                when (column) {{
            "),
        ];

        for column in &table.columns {
            let column_name = &column.name;
            let updatable_column = format!("{}Column", column_name.to_upper_camel_case());
            let bind = self.bind_single(
                column,
                &mut vec![format!("column.{}", column.name)],
                index,
                false,
            );
            let name_bind = format!("val{index}");
            let logging = if self.config.android_package_name.is_empty()
                || !self.config.android_verbose_sql_logging
            {
                "".to_string()
            } else {
                format!(
                    "
if ({}.BuildConfig.DEBUG) {{
    names.add({name_bind}?.toString() ?: \"null\")
}}
                ",
                    self.config.android_package_name
                )
            };

            index += 1;

            contents.push(format!(
                "is UpdatableColumnWithValue.{updatable_column} -> {{
                if (closures.isNotEmpty()) {{
                    updateQuery += \", \"
                }}

                updateQuery += \"{column_name} = ?\"

                closures.add{{ stmt ->
                    {bind}
                    {logging}
                }}
            }}"
            ))
        }

        contents.push(format!(
            "}}
        }}
        val finalQuery = updateQuery + \" \" + pkQuery
        val stmt = database.compileCached(finalQuery, names)

        for (closure in closures) {{
            closure(stmt)
        }}

        {bindings_pk}

        val value = stmt.executeUpdateDelete()

        if (assertOneRowAffected && value == 0) {{
            assert(false)
        }}

        return value == 1
        "
        ));

        contents.join("\n")
    }

    fn updatable_columns(&self, table: &Table) -> String {
        let mut columns_updatable_value = vec![];
        let mut columns_updatable = vec![];
        let mut switch = vec![];
        let mut mapping_to_column_without_val = vec![];

        for column in &table.columns {
            let kotlin_ty = self.kotlin_type(column);
            let column_name = &column.name;
            let class_name = column_name.to_upper_camel_case();

            columns_updatable.push(class_name.to_string());

            // Add a 'Column' suffix, else it's possible Kotlin things the argument is the sealed class instance
            // and a compile error occurs
            columns_updatable_value.push(format!("data class {class_name}Column(val {column_name}: {kotlin_ty}): UpdatableColumnWithValue()"));
            switch.push(format!(
                "is {class_name}Column -> entity.{column_name} = {column_name}"
            ));
            mapping_to_column_without_val.push(format!(
                "is {class_name}Column -> UpdatableColumn.{class_name}"
            ));
        }

        let columns_updatable_value = columns_updatable_value.join("\n");
        let mapping_to_column_without_val = mapping_to_column_without_val.join("\n");
        let columns_updatable = columns_updatable.join(",\n");
        let switch = switch.join("\n");
        let table_name = self.config.create_type_name(&table.table_name);

        format!(
            "sealed class UpdatableColumnWithValue {{
        {columns_updatable_value}
        fun update(entity: {table_name}) {{
            when (this) {{
                {switch}
            }}
        }}
       fun toUpdatableColumn(): UpdatableColumn {{
            return when (this) {{
                {mapping_to_column_without_val}
            }}
        }}
        }}
        enum class UpdatableColumn {{
        {columns_updatable}
        }}
        "
        )
    }
}
