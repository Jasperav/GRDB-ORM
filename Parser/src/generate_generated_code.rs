use crate::configuration::{Config, Visibility};
use crate::custom_mapping::CustomMapping;
use crate::parse::parse;
use grdb_orm_lib::dyn_query::DynamicQuery;
use grdb_orm_lib::room::Room;
use regex::Regex;
use sqlite_parser::Metadata;
use std::env::current_dir;
use std::fs::File;

/// Run this to fill the generated folder with the most up to date code
/// Annotated as a test so it can be executed
#[test]
fn update_generated_code() {
    let (metadata, path) = create_db(
        "
        create table User
            (
                userUuid TEXT PRIMARY KEY NOT NULL,
                firstName TEXT,
                jsonStruct TEXT NOT NULL,
                jsonStructOptional TEXT,
                jsonStructArray TEXT NOT NULL,
                jsonStructArrayOptional TEXT,
                integer INTEGER NOT NULL,
                bool INTEGER NOT NULL,
                serializedInfo BLOB NOT NULL,
                serializedInfoNullable BLOB
            );

        create table Book
            (
                bookUuid TEXT PRIMARY KEY NOT NULL,
                userUuid TEXT,
                integerOptional INTEGER,
                tsCreated INTEGER NOT NULL,
                FOREIGN KEY(userUuid) REFERENCES User(userUuid)
            );

        create table UserBook
            (
                bookUuid TEXT NOT NULL,
                userUuid TEXT NOT NULL,
                realToDouble REAL,
                PRIMARY KEY (bookUuid, userUuid),
                FOREIGN KEY(bookUuid) REFERENCES Book(bookUuid),
                FOREIGN KEY(userUuid) REFERENCES User(userUuid)
            );

        create table Parent
            (
                parentUuid TEXT NOT NULL,
                userUuid TEXT,
                PRIMARY KEY (parentUuid),
                FOREIGN KEY(userUuid) REFERENCES User(userUuid)
            );
        ",
    );

    let config = Config {
        visibility: Visibility::Public,
        output_dir: current_dir().unwrap(),
        custom_mapping: vec![
            CustomMapping {
                the_type: "[JsonType]".to_string(),
                regexes: vec![Regex::new("jsonStructArray.*").unwrap()],
            },
            CustomMapping {
                the_type: "JsonType".to_string(),
                regexes: vec![Regex::new("jsonStruct.*").unwrap()],
            },
            CustomMapping {
                the_type: "UUID".to_string(),
                regexes: vec![Regex::new(".*Uuid").unwrap()],
            },
            CustomMapping {
                the_type: "Int64".to_string(),
                regexes: vec![Regex::new("tsCreated").unwrap()],
            },
            CustomMapping {
                the_type: "Bool".to_string(),
                regexes: vec![Regex::new("bool").unwrap()],
            },
            CustomMapping {
                the_type: "SerializedInfo".to_string(),
                regexes: vec![Regex::new("serializedInfo*").unwrap()],
            },
        ],
        dynamic_queries: vec![
            DynamicQuery {
                extension: "Book".to_string(),
                func_name: "booksForUserWithSpecificUuid".to_string(),
                parameter_types: vec![(
                    "User".to_string(),
                    "userUuid".to_string(),
                    "userUuid".to_string(),
                )],
                return_types: vec![
                    "Book".to_string(),
                    "User.integer".to_string(),
                    "User.jsonStructArrayOptional".to_string(),
                    "Int".to_string(),
                ],
                return_types_is_array: true,
                query: "select Book.*, User.integer, User.jsonStructArrayOptional, 1 from Book
                    join User on User.userUuid = Book.userUuid
                    where User.userUuid = ?"
                    .to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "Book".to_string(),
                func_name: "booksWithOptionalUser".to_string(),
                parameter_types: vec![],
                return_types: vec![
                    "Book".to_string(),
                    "User?".to_string(),
                    "Bool?".to_string(),
                ],
                return_types_is_array: true,
                query: "select Book.*, User.*, Book.integerOptional
                    from Book
                    left join User on User.userUuid = Book.userUuid"
                    .to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "like".to_string(),
                parameter_types: vec![
                    ("User".to_string(), "firstName".to_string(), "firstName0".to_string()),
                    ("User".to_string(), "firstName".to_string(), "firstName1".to_string()),
                    ("User".to_string(), "firstName".to_string(), "firstName2".to_string()),
                    ("User".to_string(), "firstName".to_string(), "firstName3".to_string()),
                    ("User".to_string(), "firstName".to_string(), "firstName4".to_string()),
                ],
                return_types: vec![
                    "User".to_string(),
                ],
                return_types_is_array: true,
                query: "select User.* from User where User.firstName LIKE '%?%' or User.firstName = ? or User.firstName LIKE '%?' or User.firstName LIKE '?%' or User.firstName = ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "findByUsername".to_string(),
                parameter_types: vec![(
                    "User".to_string(),
                    "firstName".to_string(),
                    "firstName".to_string(),
                )],
                return_types: vec!["User".to_string()],
                return_types_is_array: false,
                query: "select * from User where firstName = ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "findUserUuidByUsername".to_string(),
                parameter_types: vec![(
                    "User".to_string(),
                    "firstName".to_string(),
                    "firstName".to_string(),
                )],
                return_types: vec!["User.userUuid".to_string()],
                return_types_is_array: false,
                query: "select userUuid from User where firstName = ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "amountOfUsers".to_string(),
                parameter_types: vec![],
                return_types: vec!["Int".to_string()],
                return_types_is_array: false,
                query: "select count(*) from User".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "Book".to_string(),
                func_name: "deleteByUserUuid".to_string(),
                parameter_types: vec![(
                    "Book".to_string(),
                    "userUuid".to_string(),
                    "userUuid".to_string(),
                )],
                return_types: vec![],
                return_types_is_array: false,
                query: "delete from Book where userUuid = ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "Book".to_string(),
                func_name: "hasAtLeastOneBook".to_string(),
                parameter_types: vec![],
                return_types: vec!["Bool".to_string()],
                return_types_is_array: false,
                query: "select exists(select 1 from Book)".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "serializeInfoSingle".to_string(),
                parameter_types: vec![],
                return_types: vec![
                    "User.serializedInfo".to_string(),
                    "User.serializedInfoNullable".to_string(),
                ],
                return_types_is_array: false,
                query: "select serializedInfo, serializedInfoNullable from user limit 1"
                    .to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "serializeInfoArray".to_string(),
                parameter_types: vec![],
                return_types: vec![
                    "User.serializedInfo".to_string(),
                    "User.serializedInfoNullable".to_string(),
                ],
                return_types_is_array: true,
                query: "select serializedInfo, serializedInfoNullable from user".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "serializeInfoArray".to_string(),
                parameter_types: vec![
                    ("User".to_string(), "serializedInfo".to_string(), "serializedInfo".to_string()),
                    ("User".to_string(), "serializedInfoNullable".to_string(), "serializedInfoNullable".to_string()),
                    ("User".to_string(), "firstName".to_string(), "firstName".to_string()),
                ],
                return_types: vec![],
                return_types_is_array: false,
                query: "update user set serializedInfo = ? and serializedInfoNullable = ? where firstName = ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "allWithProvidedFirstNames".to_string(),
                parameter_types: vec![
                    ("User".to_string(), "firstName".to_string(), "firstName".to_string()),
                ],
                return_types: vec!["User".to_string()],
                return_types_is_array: true,
                query: "select * from user where firstName in %PARAM_IN%".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "complex".to_string(),
                parameter_types: vec![
                    ("User".to_string(), "firstName".to_string(), "firstNames0".to_string()),
                    ("User".to_string(), "jsonStructOptional".to_string(), "jsonStructOptional".to_string()),
                    ("User".to_string(), "integer".to_string(), "integer".to_string()),
                    ("User".to_string(), "serializedInfoNullable".to_string(), "serializedInfoNullable".to_string()),
                ],
                return_types: vec!["User".to_string()],
                return_types_is_array: true,
                query: "select * from user where firstName in %PARAM_IN% and jsonStructOptional = ? and integer in %PARAM_IN% and serializedInfoNullable = ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "Parent".to_string(),
                func_name: "retrieveOptionalUserValues".to_string(),
                parameter_types: vec![
                    ("Parent".to_string(), "parentUuid".to_string(), "parentUuid".to_string()),
                ],
                return_types: vec![
                    "Parent.parentUuid".to_string(),
                    "User.userUuid?".to_string(),
                    "User.jsonStructArray?".to_string(),
                    "User.jsonStructArrayOptional".to_string(),
                ],
                return_types_is_array: true,
                query: "select parentUuid, U.userUuid, jsonStructArray, jsonStructArrayOptional from Parent left join User U on U.userUuid = Parent.userUuid where parentUuid = ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "Parent".to_string(),
                func_name: "retrieveOptionalUserValuesMapped".to_string(),
                parameter_types: vec![
                    ("Parent".to_string(), "parentUuid".to_string(), "parentUuid".to_string()),
                ],
                return_types: vec![
                    "Parent.parentUuid".to_string(),
                    "User.userUuid?".to_string(),
                    "User.jsonStructArray?".to_string(),
                    "User.jsonStructArrayOptional".to_string(),
                ],
                return_types_is_array: true,
                query: "select parentUuid, U.userUuid, jsonStructArray, jsonStructArrayOptional from Parent left join User U on U.userUuid = Parent.userUuid where parentUuid = ? order by Parent.userUuid".to_string(),
                map_to_different_type: Some("retrieveOptionalUserValues".to_string()),
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
            DynamicQuery {
                extension: "Parent".to_string(),
                func_name: "limited".to_string(),
                parameter_types: vec![
                    ("Int".to_string(), "limit".to_string(), "limit".to_string()),
                ],
                return_types: vec![
                    "Parent".to_string(),
                ],
                return_types_is_array: true,
                query: "select * from Parent limit ?".to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            },
        ],
        suffix_swift_structs: "",
        prefix_swift_structs: "Db",
        use_swiftformat: true,
        use_swiftlint: true,
        sqlite_location: path.clone(),
        all_immutable: false,
        imports: "import Foundation\nimport GRDB".to_string(),
        index_optimizer: false,
        output_dir_android: Default::default(),
        room: Room { imports: vec![], disallow_default_dao_methods: false, skip_type_converters: vec![], convert_with_gson_type_converters: vec![], unique_indexes: vec![], gson_type_adapters: vec![], },
        type_interfaces_custom_code: vec![],
        android_package_name: "".to_string(),
        android_verbose_sql_logging: false,
    };

    parse(metadata, config);

    delete_db(path);
}

pub fn create_db(batch: &str) -> (Metadata, String) {
    let db_path = current_dir().unwrap().join("generatedfortest.sqlite3");

    File::create(&db_path).unwrap();

    let con = rusqlite::Connection::open(&db_path).unwrap();

    con.execute_batch(batch).unwrap();

    let tables = sqlite_parser::parse_no_parser(&db_path);

    (tables, db_path.to_str().unwrap().to_string())
}

pub fn delete_db(path: String) {
    std::fs::remove_file(path).unwrap();
}

mod index_optimizer_test {
    use crate::generate_generated_code::create_db;
    use crate::parse::parse;
    use crate::{Config, Visibility};
    use grdb_orm_lib::dyn_query::DynamicQuery;
    use grdb_orm_lib::room::Room;
    use std::env::current_dir;

    fn setup(query: &str, add_index: bool) {
        let mut table_create = "create table User(
                userUuid TEXT PRIMARY KEY NOT NULL,
                name TEXT,
                something_random TEXT
            );"
        .to_string();

        if add_index {
            table_create += "\nCREATE INDEX user_name
            ON User (name);";
        }

        let (metadata, path) = create_db(&table_create);

        let config = Config {
            visibility: Visibility::Public,
            output_dir: current_dir().unwrap(),
            custom_mapping: vec![],
            dynamic_queries: vec![DynamicQuery {
                parameter_types: vec![],
                extension: "User".to_string(),
                func_name: "selectAll".to_string(),
                return_types: vec!["User".to_string()],
                return_types_is_array: true,
                query: query.to_string(),
                map_to_different_type: None,
                bypass_index_optimizer: false,
                ignore_query_sanitizing_android: false,
            }],
            suffix_swift_structs: "",
            prefix_swift_structs: "",
            use_swiftformat: false,
            use_swiftlint: false,
            sqlite_location: path,
            all_immutable: false,
            imports: "".to_string(),
            index_optimizer: true,
            output_dir_android: Default::default(),
            room: Room {
                imports: vec![],
                disallow_default_dao_methods: false,
                skip_type_converters: vec![],
                convert_with_gson_type_converters: vec![],
                unique_indexes: vec![],
                gson_type_adapters: vec![],
            },
            type_interfaces_custom_code: vec![],
            android_package_name: "".to_string(),
            android_verbose_sql_logging: false,
        };

        parse(metadata, config);
    }

    #[test]
    #[should_panic]
    fn test_unused_index() {
        setup("select * from User", true);
    }

    #[test]
    #[should_panic]
    fn test_missing_index() {
        setup("select * from User where name = 'test'", false);
    }

    #[test]
    fn correct() {
        setup("select * from User where name = 'test'", true);
    }

    #[test]
    fn correct_join() {
        setup(
            "select *, (select 1 from User where name = 'x') from User where name = 'test'",
            true,
        );
    }
}
