use grdb_orm_lib::dyn_query::DynamicQuery;
use grdb_orm_lib::room::Room;
use std::path::PathBuf;

use crate::custom_mapping::CustomMapping;
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
    pub use_swiftlint: bool,
    pub sqlite_location: String,
    pub all_immutable: bool,
    pub imports: String,
    pub index_optimizer: bool,
    pub output_dir_android: PathBuf,
    pub room: Room,
}

impl Config {
    pub fn create_line_writer(&self) -> LineWriter {
        let mut line_writer = LineWriter::new(
            self.visibility.modifier(),
            create_safe_dir(&self.output_dir),
        );

        self.write_imports(&mut line_writer);

        line_writer
    }

    fn write_imports(&self, line_writer: &mut LineWriter) {
        line_writer.add_line(self.imports.clone());
        line_writer.new_line();
    }

    pub fn immutability(&self) -> &'static str {
        if self.all_immutable {
            "let"
        } else {
            "var"
        }
    }

    pub fn create_type_name(&self, type_name: &str) -> String {
        format!(
            "{}{}{}",
            self.prefix_swift_structs, type_name, self.suffix_swift_structs
        )
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
