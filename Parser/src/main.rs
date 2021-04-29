use configuration::{Config, Visibility};
use std::env::current_exe;
use std::path::Path;
use std::time::Duration;

/// Easy way to read a file to a string and call a `transform` method
macro_rules! read {
    ($val: ident) => {
        pub fn read(path: std::path::PathBuf) -> Vec<$val> {
            use std::io::Read;

            crate::read_file_log(&path);

            let mut s = String::new();
            let _file = std::fs::File::open(path)
                .unwrap()
                .read_to_string(&mut s)
                .unwrap();

            transform(&s)
        }
    };
}

mod configuration;
mod custom_mapping;
mod dynamic_queries;
mod format_swift_code;
mod line_writer;
mod output_dir_initializer;
mod parse;
mod properties;
mod query_writer;
mod shared;
mod swift_property;
mod swift_struct;
mod table_meta_data;

#[cfg(test)]
mod generate_generated_code;

fn main() {
    println!("Preparing to generate Swift structs and queries...");

    std::thread::sleep(Duration::from_secs(1));

    let mut config_current_dir = current_exe().unwrap();

    println!("Trying to find config dir");

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
        "Didn't found a SQLite database at {}",
        sqlite_location
    );

    let tables = sqlite_parser::parse_no_parser(sqlite_location);
    let config = Config {
        visibility: Visibility::from_str(&*properties::VISIBILITY),
        output_dir: Path::new(&*properties::OUTPUT_DIR).to_owned(),
        custom_mapping,
        dynamic_queries,
        suffix_swift_structs: &*properties::SUFFIX_SWIFT_STRUCTS,
        prefix_swift_structs: &*properties::PREFIX_SWIFT_STRUCTS,
        use_swiftformat: *properties::USE_SWIFTFORMAT,
        sqlite_location: properties::SQLITE_LOCATION.to_owned(),
        all_immutable: *properties::ALL_IMMUTABLE,
    };

    println!("Successfully parsed configuration files");

    parse::parse(tables, config);

    println!("Successfully generated Swift structs and queries!");
}

fn read_file_log(file: &Path) {
    let file_name = file.file_name().unwrap().to_str().unwrap();

    println!("Reading file: {}", file_name);
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => panic!(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
