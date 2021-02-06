use crate::configuration::{Config, Visibility};
use crate::custom_mapping::CustomMapping;
use crate::dynamic_queries::DynamicQuery;
use crate::parse::parse;
use regex::Regex;
use rusqlite::NO_PARAMS;
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
                return_types: vec!["User?".to_string()],
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
                return_types: vec!["User.userUuid?".to_string()],
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
        ],
        suffix_swift_structs: "",
        prefix_swift_structs: "Db",
        use_swiftformat: true,
        sqlite_location: path.clone(),
        all_immutable: false,
    };

    parse(metadata, config);

    delete_db(path);
}

pub fn create_db() -> (Metadata, String) {
    let db_path = current_dir().unwrap().join("generatedfortest.sqlite3");

    File::create(&db_path).unwrap();

    let con = rusqlite::Connection::open(&db_path).unwrap();

    con.execute(
        "
        create table User
            (
                userUuid TEXT PRIMARY KEY NOT NULL,
                firstName TEXT,
                jsonStruct TEXT NOT NULL,
                jsonStructOptional TEXT,
                jsonStructArray TEXT NOT NULL,
                jsonStructArrayOptional TEXT,
                integer INTEGER NOT NULL
            );
        ",
        NO_PARAMS,
    )
    .unwrap();

    con.execute(
        "
            create table Book
            (
                bookUuid TEXT PRIMARY KEY NOT NULL,
                userUuid TEXT,
                integerOptional INTEGER,
                FOREIGN KEY(userUuid) REFERENCES User(userUuid)
            );",
        NO_PARAMS,
    )
    .unwrap();

    let tables = sqlite_parser::parse_no_parser(&db_path);

    (tables, db_path.to_str().unwrap().to_string())
}

pub fn delete_db(path: String) {
    std::fs::remove_file(path).unwrap();
}
