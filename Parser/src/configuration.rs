use std::path::PathBuf;

use crate::custom_mapping::CustomMapping;
use crate::dynamic_queries::DynamicQuery;
use crate::line_writer::LineWriter;
use crate::output_dir_initializer::create_safe_dir;

/// All the configuration that can be done in one place
pub struct Config {
    pub visibility: Visibility,
    pub output_dir: PathBuf,
    pub custom_mapping: Vec<CustomMapping>,
    pub dynamic_queries: Vec<DynamicQuery>,
    pub suffix_swift_structs: &'static str,
    pub prefix_swift_structs: &'static str,
    pub use_swiftformat: bool,
    pub sqlite_location: String,
    pub all_immutable: bool,
}

impl Config {
    pub fn create_line_writer(&self) -> LineWriter {
        LineWriter::new(
            self.visibility.modifier(),
            create_safe_dir(&self.output_dir),
        )
    }

    pub fn immutability(&self, is_pk: bool) -> &'static str {
        if self.all_immutable || is_pk {
            "let"
        } else {
            "var"
        }
    }
}

/// The visibility of the Swift type/property
pub enum Visibility {
    Public,
    Internal,
}

impl Visibility {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "public" => Self::Public,
            "internal" => Self::Internal,
            _ => panic!("Visibility must be either Public or Internal"),
        }
    }

    pub fn modifier(&self) -> &'static str {
        match self {
            Visibility::Public => "public ",
            Visibility::Internal => "",
        }
    }
}
