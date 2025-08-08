use clap::Parser;
pub use configuration::{Config, Visibility};
use sqlite_parser::{Column, Table};
use std::env::current_exe;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

/// Easy way to read a file to a string and call a `transform` method
macro_rules! read {
    ($val: ty) => {
        pub fn read(path: std::path::PathBuf) -> $val {
            crate::read_file_log(&path);

            let content = std::fs::read_to_string(path).unwrap();

            transform(&content)
        }
    };
}

mod configuration;
mod custom_mapping;
mod dynamic_queries;
mod format_swift_code;
mod line_writer;
mod metadata;
mod output_dir_initializer;
mod parse;
mod properties;
mod query_writer;
mod shared;
mod swift_property;
mod swift_struct;
mod table_meta_data;

pub mod android;
#[cfg(test)]
mod generate_generated_code;
mod room;
mod type_interfaces_custom_code;

pub const SET_ARGUMENTS: &str = "#if DEBUG\ntry statement.setArguments(arguments)\n#else\nstatement.setUncheckedArguments(arguments)\n#endif";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    location_config: Option<String>,
}

fn main() {
    println!("Preparing to generate Swift structs and queries...");

    let args = Args::parse();

    std::thread::sleep(Duration::from_secs(1));

    let mut config_current_dir = if let Some(config) = args.location_config {
        println!("Found explicit config file");

        PathBuf::from_str(&config).unwrap()
    } else {
        println!("Did not found an explicit config file, trying to find one recursively");

        current_exe().unwrap()
    };

    println!("Trying to find config dir, starting at {config_current_dir:#?} and moving up");

    loop {
        let joined_config = config_current_dir.join("config");

        if joined_config.exists() {
            config_current_dir = joined_config;

            break;
        } else {
            config_current_dir = config_current_dir
                .parent()
                .expect("Couldn't find config folder")
                .to_path_buf();
        }
    }

    let env_file = config_current_dir.join(".env");

    read_file_log(&env_file);

    dotenv::from_filename(env_file).unwrap();

    let custom_mapping =
        crate::custom_mapping::read(config_current_dir.join("custom_mapping.toml"));
    let dynamic_queries = crate::dynamic_queries::read(config_current_dir.join("dyn_queries.toml"));
    let sqlite_location = &*properties::SQLITE_LOCATION;

    assert!(
        Path::new(sqlite_location).exists(),
        "Didn't found a SQLite database at {sqlite_location}"
    );

    let tables = sqlite_parser::parse_no_parser(sqlite_location);
    let packages = (*properties::PACKAGES).clone() + "|Foundation|GRDB";
    let packages = packages
        .split('|')
        .filter(|s| s != &"|")
        .map(|i| format!("import {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let config = Config {
        visibility: Visibility::from_str_ok(&properties::VISIBILITY),
        output_dir: Path::new(&*properties::OUTPUT_DIR).to_owned(),
        custom_mapping,
        dynamic_queries,
        suffix_swift_structs: &properties::SUFFIX_SWIFT_STRUCTS,
        prefix_swift_structs: &properties::PREFIX_SWIFT_STRUCTS,
        use_swiftformat: *properties::USE_SWIFTFORMAT,
        use_swiftlint: *properties::USE_SWIFTLINT,
        sqlite_location: properties::SQLITE_LOCATION.to_owned(),
        all_immutable: *properties::ALL_IMMUTABLE,
        imports: packages,
        index_optimizer: *properties::INDEX_OPTIMIZER,
        output_dir_android: Path::new(&*properties::OUTPUT_DIR_ANDROID).to_owned(),
        room: crate::room::read(config_current_dir.join("room.toml")),
        type_interfaces_custom_code: crate::type_interfaces_custom_code::read(
            config_current_dir.join("type_interfaces_custom_code.toml"),
        ),
        android_package_name: properties::ANDROID_PACKAGE_NAME.to_owned(),
        android_verbose_sql_logging: *properties::ANDROID_ENABLE_VERBOSE_SQL_ARGUMENT_LOGGING,
    };

    println!("Successfully parsed configuration files");

    parse::parse(tables, config);

    println!("Successfully generated Swift structs and queries!");
}

fn read_file_log(file: &Path) {
    let file_name = file.file_name().unwrap().to_str().unwrap();

    println!("Reading file: {file_name}");
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => panic!(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn primary_keys(table: &Table) -> Vec<&Column> {
    table
        .columns
        .iter()
        .filter(|c| c.part_of_pk)
        .collect::<Vec<_>>()
}
