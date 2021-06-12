use crate::configuration::{Config, Visibility};
use crate::custom_mapping::CustomMapping;
use crate::dynamic_queries::DynamicQuery;
use crate::parse::parse;
use crate::upsert::Upsert;
use regex::Regex;
use sqlite_parser::Metadata;
use std::env::current_dir;
use std::fs::File;

/// Run this to fill the generated folder with the most up to date code
/// Annotated as a test so it can be executed
#[test]
fn update_generated_code() {
    let (metadata, path) = create_db();

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
            },
            DynamicQuery {
                extension: "User".to_string(),
                func_name: "amountOfUsers".to_string(),
                parameter_types: vec![],
                return_types: vec!["Int".to_string()],
                return_types_is_array: false,
                query: "select count(*) from User".to_string(),
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
            },
            DynamicQuery {
                extension: "Book".to_string(),
                func_name: "hasAtLeastOneBook".to_string(),
                parameter_types: vec![],
                return_types: vec!["Bool".to_string()],
                return_types_is_array: false,
                query: "select exists(select 1 from Book)".to_string(),
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
                query: "select * from user where firstName in %PARAM_IN% and jsonStructOptional = ? and firstName in %PARAM_IN% and firstName = ?".to_string(),
            },
        ],
        upserts: vec![Upsert {
            table: "User".to_string(),
            columns_to_update: vec![
                "jsonStruct".to_string(),
                "jsonStructOptional".to_string(),
                "integer".to_string(),
            ],
            func_name: "upsertExample".to_string(),
        }],
        suffix_swift_structs: "",
        prefix_swift_structs: "Db",
        use_swiftformat: true,
        sqlite_location: path.clone(),
        all_immutable: false,
        imports: "import Foundation\nimport GRDB".to_string(),
    };

    parse(metadata, config);

    delete_db(path);
}

pub fn create_db() -> (Metadata, String) {
    let db_path = current_dir().unwrap().join("generatedfortest.sqlite3");

    File::create(&db_path).unwrap();

    let con = rusqlite::Connection::open(&db_path).unwrap();

    con.execute_batch(
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
        ",
    )
    .unwrap();

    let tables = sqlite_parser::parse_no_parser(&db_path);

    (tables, db_path.to_str().unwrap().to_string())
}

pub fn delete_db(path: String) {
    std::fs::remove_file(path).unwrap();
}
