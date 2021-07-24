use crate::swift_property::SwiftProperty;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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

pub fn parameter_types_separated_colon(pt: &[&SwiftProperty]) -> String {
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

    pub fn add_line<T: ToString>(&mut self, line: T) {
        self.lines.push(line.to_string());
    }

    pub fn add_comment(&mut self, comment: &str) {
        self.lines.push(format!("// {}", comment));
    }

    pub fn add_closing_brackets(&mut self) {
        self.lines.push("}\n".to_string());
    }

    pub fn add_with_modifier<T: ToString>(&mut self, t: T) {
        self.lines
            .push(format!("{} {}", self.modifier, t.to_string()));
    }

    pub fn write_to_file(self, file_name: &str) {
        let mut file = File::create(self.output_dir.join(format!("{}.swift", file_name))).unwrap();

        for line in self.lines {
            writeln!(file, "{}", line).unwrap();
        }
    }
}
