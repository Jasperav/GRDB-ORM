use crate::android::{generate_kotlin_package, SUPPRESS_ALL};
use crate::configuration::Config;
use crate::custom_mapping::CustomMapping;
use crate::primary_keys;
use grdb_orm_lib::dyn_query::DynamicQuery;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use regex::Regex;
use sqlite_parser::{Column, Metadata, OnUpdateAndDelete, Type};
use std::fs::File;
use std::path::Path;
use std::process::Command;

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

        let entity = self.config.output_dir_android.join("entity");

        // Don't use the generated suffix here, stupid packages...
        let _ = std::fs::remove_dir_all(&entity);

        // Create the folder to put the generated files in
        std::fs::create_dir_all(&entity).unwrap();

        let imports = self
            .config
            .room
            .imports
            .iter()
            .map(|a| format!("import {a}"))
            .collect::<Vec<_>>()
            .join("\n")
            + "\nimport java.util.*\nimport androidx.room.*";
        let dyn_queries = self.generate_dyn_queries(&entity, &imports);
        let mappers = self.generate_type_converters(&entity, &imports);
        let entities = self.generate_tables(&entity, &imports);
        let daos = self.generate_daos(&entity, &imports, dyn_queries);

        self.generate_database(&entity, &mappers, &entities, &daos);

        Command::new("ktlint")
            .arg("-F")
            .current_dir(&self.config.output_dir_android)
            .status()
            .unwrap();
    }

    pub fn interfaces_for_ty(&self, ty: &str) -> String {
        let t = match self
            .config
            .type_interfaces_custom_code
            .iter()
            .find(|t| t.ty == ty)
        {
            None => return " {".to_string(),
            Some(t) => t,
        };

        let interfaces = t.interfaces.join(", ");

        format!(": {interfaces} {{\n{}\n", t.custom_code)
    }

    pub fn convert_parameter_type_to_kotlin_type(&self, table: &str, column: &str) -> String {
        if table == "Int" {
            "Int".to_string()
        } else {
            self.kotlin_type_from_table_column(table, column)
        }
    }

    fn generate_dyn_queries(&self, path: &Path, imports: &str) -> Vec<DynQueryToWriteInDao> {
        let package = generate_kotlin_package(path);
        let mut dyn_queries = vec![SUPPRESS_ALL.to_string(), package, imports.to_string()];
        let mut to_write_in_dao = vec![];

        for dyn_query in &self.config.dynamic_queries {
            // https://stackoverflow.com/a/75465896/7715250
            // The query needs to be sanitezed...
            let start_query = self.sanitize_query(dyn_query);
            let mut query = format!("@Query(\"{}\")", start_query);

            macro_rules! create_ty {
                ($t: expr) => {
                    format!("{}{}", dyn_query.extension, $t.to_upper_camel_case())
                };
            }

            let mut arguments = vec![];
            let (ty, write_type) = if let Some(t) = &dyn_query.map_to_different_type {
                (create_ty!(t), false)
            } else {
                (create_ty!(dyn_query.func_name), true)
            };

            for (table, column, arg) in &dyn_query.parameter_types {
                let kotlin_type = self.convert_parameter_type_to_kotlin_type(table, column);

                // Replace every ? with the corresponding placeholder
                query = query.replacen('?', &format!(":{arg}"), 1);

                arguments.push(format!("{arg}: {kotlin_type}"));
            }

            assert!(query.find('?').is_none());

            // https://stackoverflow.com/questions/44184769/android-room-select-query-with-like
            query = query.replace("%'", " || '%'").replace("%:", "%' || :");

            let inner = if dyn_query.return_types_is_array {
                format!("List<{ty}>")
            } else {
                format!("{ty}?")
            };
            let (prefix, suffix, func_suffix) = if dyn_query.return_types.is_empty() {
                ("suspend ", "".to_string(), "")
            } else {
                ("", format!(": LiveData<{inner}>"), "Tracked")
            };

            let mut tracked = format!(
                "{prefix}fun {}{func_suffix}({}){suffix}",
                dyn_query.func_name,
                arguments.join(", ")
            );

            if dyn_query.return_types.is_empty() {
                let untracked_blocking = format!(
                    "\n{query}\nfun {}Blocking({})",
                    dyn_query.func_name,
                    arguments.join(", ")
                );

                tracked += &untracked_blocking;
            } else {
                let untracked = format!(
                    "\n{query}\nsuspend fun {}({}): {inner}",
                    dyn_query.func_name,
                    arguments.join(", ")
                );
                let untracked_blocking = format!(
                    "\n{query}\nfun {}Blocking({}): {inner}",
                    dyn_query.func_name,
                    arguments.join(", ")
                );

                tracked += &(untracked + &untracked_blocking)
            }

            to_write_in_dao.push(DynQueryToWriteInDao {
                query: query + "\n" + &tracked,
                table: dyn_query.extension.clone(),
            });

            if !write_type || dyn_query.return_types.is_empty() {
                continue;
            }

            let mut return_types = vec![];

            for (index, return_ty) in dyn_query.return_types.iter().enumerate() {
                let mut return_ty = return_ty.clone();
                let nullable = if return_ty.contains('?') { "?" } else { "" };

                return_ty = return_ty.replace('?', "");

                let table_column = return_ty.split('.').collect::<Vec<_>>();

                let (field, embedded) = if table_column.len() == 1 {
                    if table_column[0] == "Int" {
                        ("Int".to_string(), "".to_string())
                    } else {
                        let prefix = if dyn_query.return_types.len() == 1 {
                            "".to_string()
                        } else {
                            format!("(prefix = \"arg{}\")", index)
                        };

                        (
                            self.config.create_type_name(table_column[0]),
                            format!("@Embedded{prefix}\n"),
                        )
                    }
                } else {
                    assert_eq!(2, table_column.len());

                    let kotlin_type =
                        self.kotlin_type_from_table_column(table_column[0], table_column[1]);
                    let info = format!("@ColumnInfo(name = \"arg{index}{}\")\n", table_column[1]);

                    (kotlin_type, info)
                };

                return_types.push(format!(
                    "{embedded}val arg{}: {}{nullable}",
                    return_types.len(),
                    field
                ))
            }

            let custom_code = self.interfaces_for_ty(&ty);

            dyn_queries.push(format!(
                "data class {ty}(\n{}) {custom_code}}}",
                return_types.join(",\n")
            ))
        }

        std::fs::write(path.join("DynQueries.kt"), dyn_queries.join("\n")).unwrap();

        to_write_in_dao
    }

    fn generate_daos(
        &self,
        path: &Path,
        imports: &str,
        dyn_queries: Vec<DynQueryToWriteInDao>,
    ) -> Vec<(String, String)> {
        let mut daos = vec![];
        let package = generate_kotlin_package(path);

        for table in self.metadata.tables.values() {
            let table_name = &table.table_name;
            let type_name = self.config.create_type_name(table_name);
            let dao = format!("{type_name}Dao");
            let path = path.join(format!("{dao}.kt"));
            let mut pk_in_query = vec![];
            let mut pk_in_method = vec![];

            for pk in primary_keys(table) {
                pk_in_query.push(format!("{p} = :{p}", p = pk.name));
                pk_in_method.push(format!("{}: {}", pk.name, self.kotlin_type(pk)));
            }

            let pk_in_method = pk_in_method.join(", ");
            let pk_in_query = pk_in_query.join(" and ");
            let select_all_raw = format!("SELECT * FROM {table_name}");
            let select_all = format!("@Query(\"{select_all_raw}\")");
            let select_unique = format!("@Query(\"{select_all_raw} where {pk_in_query}\")");
            let count_query = format!("@Query(\"select count(1) from {table_name}\")");
            let exists_unique_query = format!(
                "@Query(\"select exists(select 1 from {table_name} where {pk_in_query})\")"
            );
            let mut content = vec![format!(
                "
{SUPPRESS_ALL}

                {package}


import androidx.lifecycle.LiveData
{imports}

                @Dao
                interface {dao} {{
                @Delete
                suspend fun deleteUnique(entity: {type_name})
                @Delete
                fun deleteUniqueBlocking(entity: {type_name})
                @Query(\"delete from {table_name}\")
                suspend fun deleteAll()
                @Query(\"delete from {table_name}\")
                fun deleteAllBlocking()
                @Insert
                suspend fun insert(entity: {type_name})
                @Insert
                fun insertBlocking(entity: {type_name})
                {count_query}
                fun countAll(): Int
                @Update
                suspend fun updateUnique(entity: {type_name}): Int
                @Update
                fun updateUniqueBlocking(entity: {type_name}): Int
                {select_all}
                suspend fun selectAll(): Array<{type_name}>
                {select_all}
                fun selectAllBlocking(): Array<{type_name}>
                {select_all}
                fun selectAllTrack(): LiveData<Array<{type_name}>>
                {exists_unique_query}
                fun existsUnique({pk_in_method}) : Boolean
                {select_unique}
                suspend fun selectUnique({pk_in_method}): {type_name}?
                {select_unique}
                fun selectUniqueBlocking({pk_in_method}): {type_name}?
                {select_unique}
                fun selectUniqueExpectBlocking({pk_in_method}): {type_name}
                {select_unique}
                fun selectUniqueTrack({pk_in_method}): LiveData<{type_name}?>
                "
            )];

            for dyn_query in &dyn_queries {
                if &dyn_query.table == table_name {
                    content.push(dyn_query.query.clone());
                }
            }

            content.push("}".to_string());

            std::fs::write(path, content.join("\n")).unwrap();

            // Don't use prefix/suffix here, it looks better when calling the dao functions
            daos.push((table_name.to_string().to_lower_camel_case() + "Dao", dao));
        }

        daos
    }

    fn generate_type_converters(&self, path: &Path, imports: &str) -> String {
        if self.config.custom_mapping.is_empty() {
            return "".to_string();
        }

        let package = generate_kotlin_package(path);
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

            let (parse_from, parse_to, ty) = if self
                .config
                .room
                .convert_with_gson_type_converters
                .contains(&mapping.the_type)
            {
                let convert = if self
                    .config
                    .room
                    .gson_type_adapters
                    .iter()
                    .any(|a| a[0] == kotlin_type)
                {
                    // This is needed, else some weird END_DOCUMENT errors will appear
                    format!("com.google.gson.stream.JsonReader(java.io.StringReader(it)), com.google.gson.reflect.TypeToken.get({kotlin_type}::class.java)")
                } else {
                    format!("it, {kotlin_type}::class.java")
                };

                (
                    format!("gson.fromJson({convert})"),
                    "gson.toJson(value)".to_string(),
                    "String",
                )
            } else if mapping.the_type == "UUID" {
                (
                    "UUID.fromString(it)".to_string(),
                    "value.toString()".to_string(),
                    "String",
                )
            } else {
                (
                    format!("{kotlin_type}.parseFrom(it)"),
                    "value.toByteArray()".to_string(),
                    "ByteArray",
                )
            };

            let name = format!("Converter{}", kotlin_type);

            mappers.push(TypeConverter {
                name: name.to_string(),
                to_write: format!(
                    "class {name} {{
    @TypeConverter
    fun from(value: {ty}?): {kotlin_type}? {{
        return value?.let {{ {parse_from} }}
    }}

    @TypeConverter
    fun to(value: {kotlin_type}?): {ty}? {{
        if (value == null) {{
return null
           }}

        return {parse_to}
    }}
}}"
                ),
            });
        }

        let to_write = mappers
            .iter()
            .map(|m| m.to_write.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let mut gson = "com.google.gson.GsonBuilder()\n".to_string();

        for adapter in &self.config.room.gson_type_adapters {
            assert_eq!(adapter.len(), 2);

            gson += &format!(
                ".registerTypeAdapter({}::class.java, {}())\n",
                adapter[0], adapter[1]
            );
        }

        gson += ".create()";

        let imports = format!(
            "{SUPPRESS_ALL}\n{package}import androidx.room.TypeConverter\n{imports}\nval gson = {gson}\n{to_write}"
        );

        std::fs::write(converters_path, imports).unwrap();

        let mapped = mappers
            .into_iter()
            .map(|m| format!("{}::class", m.name))
            .collect::<Vec<_>>()
            .join(", ");

        format!("@TypeConverters({mapped})")
    }

    fn generate_database(
        &self,
        path: &Path,
        converters: &str,
        entities: &[String],
        daos: &[(String, String)],
    ) {
        let package = generate_kotlin_package(path);
        let db = path.join("GeneratedDatabase.kt");

        File::create(&db).unwrap();

        let entities = entities
            .iter()
            .map(|t| format!("{t}::class"))
            .collect::<Vec<_>>()
            .join(",\n");
        let daos = daos
            .iter()
            .map(|(dao_method_name, dao)| format!("abstract fun {dao_method_name}(): {dao}"))
            .collect::<Vec<_>>()
            .join("\n");
        let logging = if self.config.android_package_name.is_empty() {
            "".to_string()
        } else {
            let query_logging = if self.config.android_verbose_sql_logging {
                "var queryFormatted = query
assert(query.count { it == '?' } == arguments.size)

    for (arg in arguments.reversed()) {
        // Make sure that this doesn't change the wrong values
        val s = arg?.toString()?.replace(\"?\", \"!QUESTION_MARK_REPLACED!\") ?: \"null\"
        val lastIndex = queryFormatted.lastIndexOf(\"?\")

        queryFormatted = queryFormatted.replaceRange(lastIndex, lastIndex + 1, s)
    }

    assert(queryFormatted.count { it == '?' } == 0)"
            } else {
                "
                // Note that you can enable verbose argument logging but this will slow down your app
                val queryFormatted = query"
            };

            format!("
if ({}.BuildConfig.DEBUG) {{
    {query_logging}

    logger.log(Level.INFO, \"Will execute query, cached: ${{existing != null}}, query: $queryFormatted\")
}}
            ", self.config.android_package_name)
        };

        let contents = format!(
            "
{SUPPRESS_ALL}

{package}

import androidx.room.Database
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
import androidx.sqlite.db.SupportSQLiteStatement
import java.util.logging.Level
import java.util.logging.Logger

        @Database(entities = [\n{entities}\n], version = 1)
{converters}
            abstract class GeneratedDatabase : RoomDatabase() {{
                val logger = Logger.getLogger(GeneratedDatabase::class.java.name)
                private val cache = hashMapOf<String, SupportSQLiteStatement>()

                fun compileCached(query: String, vararg arguments: Any?): SupportSQLiteStatement {{
                    assert(inTransaction())

                    val existing = cache[query]

                    {logging}

                    if (existing != null) {{
                        existing.clearBindings()

                        return existing
                    }}

                    val stmt = compileStatement(query)

                    cache[query] = stmt

                    return stmt
                }}
                {daos}
            }}
        "
        );

        std::fs::write(db, contents).unwrap();
    }

    pub fn custom_mapping_matches(&self, column: &Column) -> Option<&'a CustomMapping> {
        self.config
            .custom_mapping
            .iter()
            .find(|c| c.regexes.iter().any(|regex| regex.is_match(&column.name)))
    }

    pub(crate) fn kotlin_type(&self, column: &Column) -> String {
        let mut value = None;

        if let Some(v) = self.custom_mapping_matches(column) {
            value = Some(v.the_type.to_string());
        }

        let new_value = if let Some(val) = value {
            let result = match val.as_str() {
                "Bool" => "Boolean".to_string(),
                "Int64" => "Long".to_string(),
                // Special value
                "meta" => "ByteArray".to_string(),
                _ => self.convert_swift_type_to_kotlin_type(&val),
            };

            Some(result)
        } else if column.name == "meta" {
            // Special value
            Some("ByteArray".to_string())
        } else {
            None
        };

        let mut result = match new_value {
            None => match column.the_type {
                Type::Text | Type::String => "String".to_string(),
                Type::Integer => "Int".to_string(),
                Type::Real => "Double".to_string(),
                Type::Blob => panic!(
                    "Should already been mapped: {:#?}, tables: {:#?}",
                    column, self.metadata.tables
                ),
            },
            Some(val) => val,
        };

        if column.nullable {
            result += "?"
        }

        result
    }

    pub fn kotlin_column_from_table_column(&self, table: &str, column: &str) -> Column {
        self.metadata
            .tables
            .get(table)
            .unwrap_or_else(|| panic!("Table not found: {table}"))
            .columns
            .iter()
            .find(|c| c.name == column)
            .unwrap()
            .clone()
    }

    fn kotlin_type_from_table_column(&self, table: &str, column: &str) -> String {
        let column = self.kotlin_column_from_table_column(table, column);

        self.kotlin_type(&column)
    }

    fn convert_swift_type_to_kotlin_type(&self, swift_type: &str) -> String {
        let split = swift_type.split('_').collect::<Vec<_>>();

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

    pub(crate) fn convert_to_foreign_key(
        &self,
        on_update_and_delete: OnUpdateAndDelete,
    ) -> &'static str {
        match on_update_and_delete {
            OnUpdateAndDelete::NoAction => "NO_ACTION",
            OnUpdateAndDelete::Restrict => "RESTRICT",
            OnUpdateAndDelete::SetNull => "SET_NULL",
            OnUpdateAndDelete::SetDefault => "SET_DEFAULT",
            OnUpdateAndDelete::Cascade => "CASCADE",
        }
    }

    fn sanitize_query(&self, dyn_query: &DynamicQuery) -> String {
        if dyn_query.return_types.len() <= 1 {
            return dyn_query.query.clone();
        }

        let regex = Regex::new(r"(\w+)\.").unwrap();
        let mut matches = regex.captures_iter(&dyn_query.query);
        let mut new_select_clause = vec![];

        for (index, return_type) in dyn_query.return_types.iter().enumerate() {
            let table_name_in_query = matches.next().unwrap().get(1).unwrap().as_str();

            // Remove the nullable modifier, it's not needed
            let return_type = return_type.replace('?', "");
            let whole_table = return_type.split('.').collect::<Vec<_>>();
            let column_filter = if whole_table.len() == 1 {
                None
            } else {
                Some(whole_table[1].to_string())
            };

            let table = whole_table[0];
            let table = self.metadata.tables.get(table).expect(table);
            let mut new_select = vec![];

            for column in &table.columns {
                if let Some(column_filter) = &column_filter {
                    if column_filter != &column.name {
                        continue;
                    }
                }

                let column_name = &column.name;

                new_select.push(format!(
                    "{table_name_in_query}.{column_name} as arg{index}{column_name}"
                ));
            }

            let select_clause = new_select.join(", ");

            new_select_clause.push(select_clause)
        }

        let final_query = &dyn_query.query;
        let split = final_query.splitn(2, " from ").collect::<Vec<_>>();

        assert_eq!(2, split.len());

        let after_from = split[1];
        let new_select = "select ".to_string() + &new_select_clause.join(", ") + " from ";

        new_select + after_from
    }
}

struct DynQueryToWriteInDao {
    query: String,
    table: String,
}
