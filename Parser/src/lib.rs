pub fn read_file_log(file: &Path) {
    let file_name = file.file_name().unwrap().to_str().unwrap();

    println!("Reading file: {file_name}");
}

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

pub use configuration::{Config, Visibility};
use sqlite_parser::{Column, Table};
use std::path::Path;

mod configuration;
pub mod custom_mapping;
pub mod dynamic_queries;
mod format_swift_code;
mod line_writer;
mod metadata;
mod output_dir_initializer;
mod parse;
pub mod properties;
mod query_writer;
mod shared;
mod swift_property;
mod swift_struct;
mod table_meta_data;

pub mod android;
pub mod dyn_query;
#[cfg(test)]
mod generate_generated_code;
pub mod room;
pub mod type_interfaces_custom_code;

pub const SET_ARGUMENTS: &str = "#if DEBUG\ntry statement.setArguments(arguments)\n#else\nstatement.setUncheckedArguments(arguments)\n#endif";

pub fn parse(config: Config) {
    let tables = sqlite_parser::parse_no_parser(&config.sqlite_location);

    parse::parse(tables, config);
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
