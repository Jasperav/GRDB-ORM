use clap::Parser;
use sqlite_parser_swift_grdb::properties;
use std::env::current_exe;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    location_config: Option<String>,
}

fn main() {
    println!("Preparing to generate Swift structs and queries...");

    let args = Args::parse();

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

    sqlite_parser_swift_grdb::read_file_log(&env_file);

    dotenv::from_filename(env_file).unwrap();

    let custom_mapping = sqlite_parser_swift_grdb::custom_mapping::read(
        config_current_dir.join("custom_mapping.toml"),
    );
    let dynamic_queries = sqlite_parser_swift_grdb::dynamic_queries::read(
        config_current_dir.join("dyn_queries.toml"),
    );
    let sqlite_location = &*properties::SQLITE_LOCATION;

    assert!(
        Path::new(sqlite_location).exists(),
        "Didn't found a SQLite database at {sqlite_location}"
    );

    let packages = (*properties::PACKAGES).clone() + "|Foundation|GRDB";
    let packages = packages
        .split('|')
        .filter(|s| s != &"|")
        .map(|i| format!("import {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let config = sqlite_parser_swift_grdb::Config {
        visibility: sqlite_parser_swift_grdb::Visibility::from_str_ok(&properties::VISIBILITY),
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
        room: sqlite_parser_swift_grdb::room::read(config_current_dir.join("room.toml")),
        type_interfaces_custom_code: sqlite_parser_swift_grdb::type_interfaces_custom_code::read(
            config_current_dir.join("type_interfaces_custom_code.toml"),
        ),
        android_package_name: properties::ANDROID_PACKAGE_NAME.to_owned(),
        android_verbose_sql_logging: *properties::ANDROID_ENABLE_VERBOSE_SQL_ARGUMENT_LOGGING,
    };

    println!("Successfully parsed configuration files");

    sqlite_parser_swift_grdb::parse(config);

    println!("Successfully generated Swift structs and queries!");
}
