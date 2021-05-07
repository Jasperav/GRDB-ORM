use crate::swift_property::SwiftProperty;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone)]
pub enum WriteRead {
    Write,
    // Return type
    Read(String),
}

impl WriteRead {
    pub fn to_str(&self) -> &'static str {
        match self {
            WriteRead::Write => "write",
            WriteRead::Read(_) => "read",
        }
    }

    pub fn database_reader_or_writer(&self) -> &'static str {
        match self {
            WriteRead::Write => "dbWriter",
            WriteRead::Read(_) => "dbReader",
        }
    }

    pub fn generic_type(&self) -> &'static str {
        match self {
            WriteRead::Write => "DatabaseWriter",
            WriteRead::Read(_) => "DatabaseReader",
        }
    }

    pub fn return_type(&self) -> String {
        match &self {
            WriteRead::Write => "".to_string(),
            WriteRead::Read(rt) => format!("-> {}", rt),
        }
    }
}

/// Wrapper around a Vec<String>
/// Eventually all strings inside the vec will be written to a file, separated by a newline
#[derive(Debug)]
pub struct LineWriter {
    lines: Vec<String>,
    // The modifier to apply to Swift types/properties
    pub modifier: &'static str,
    // The output dir to put the generated files into
    output_dir: PathBuf,
}

#[derive(Copy, Clone)]
pub enum StaticInstance {
    Static,
    Instance,
}

impl StaticInstance {
    pub fn modifier(&self) -> &'static str {
        match self {
            StaticInstance::Static => "static ",
            StaticInstance::Instance => "",
        }
    }
}

pub fn parameter_types_separated_colon(pt: &Vec<&SwiftProperty>) -> String {
    if pt.is_empty() {
        return "".to_string();
    }

    ", ".to_string()
        + &pt
            .iter()
            .map(|pt| format!("{}: {}", pt.swift_property_name, pt.swift_type.type_name))
            .collect::<Vec<_>>()
            .join(", ")
}

fn parameter_separated_colon(pt: &Vec<&SwiftProperty>) -> String {
    if pt.is_empty() {
        return "".to_string();
    }

    ", ".to_string()
        + &pt
            .iter()
            .map(|pt| format!("{}: {}", pt.swift_property_name, pt.swift_property_name))
            .collect::<Vec<_>>()
            .join(", ")
}

impl LineWriter {
    pub fn new(modifier: &'static str, output_dir: PathBuf) -> Self {
        let mut s = Self {
            lines: Vec::new(),
            modifier,
            output_dir,
        };

        s.add_comment("// This file is generated, do not edit\n");

        s
    }

    pub fn new_line(&mut self) {
        self.lines.push("\n".to_string());
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn add_comment(&mut self, comment: &str) {
        self.lines.push(format!("// {}", comment));
    }

    pub fn add_closing_brackets(&mut self) {
        self.lines.push("}\n".to_string());
    }

    pub fn add_with_modifier(&mut self, t: String) {
        self.lines.push(format!("{} {}", self.modifier, t));
    }

    pub fn add_wrapper_pool(
        &mut self,
        static_instance: StaticInstance,
        original_method: &str,
        write_read: WriteRead,
        parameter_with_types: &Vec<&SwiftProperty>,
    ) {
        let colon_separated_parameter_types_separated =
            parameter_types_separated_colon(parameter_with_types);
        let colon_separated_parameter_separated = parameter_separated_colon(parameter_with_types);

        self.lines.push(format!(
            "{} {}func gen{}<T: {}>({}: T{}) throws {}{{\n",
            self.modifier,
            static_instance.modifier(),
            original_method,
            write_read.generic_type(),
            write_read.database_reader_or_writer(),
            colon_separated_parameter_types_separated,
            write_read.return_type()
        ));
        self.lines.push(format!(
            "try {}.{} {{ database in\ntry gen{}(db: database{})\n}}\n}}",
            write_read.database_reader_or_writer(),
            write_read.to_str(),
            original_method,
            colon_separated_parameter_separated
        ));
    }

    pub fn write_to_file(self, file_name: &str) {
        let mut file = File::create(self.output_dir.join(format!("{}.swift", file_name))).unwrap();

        for line in self.lines {
            writeln!(file, "{}", line).unwrap();
        }
    }
}
