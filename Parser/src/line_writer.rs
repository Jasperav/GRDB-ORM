use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub enum WriteRead {
    Write,
    Read
}

/// Wrapper around a Vec<String>
/// Eventually all strings inside the vec will be written to a file, separated by a newline
#[derive(Debug)]
pub struct LineWriter {
    lines: Vec<String>,
    // The modifier to apply to Swift types/properties
    modifier: &'static str,
    // The output dir to put the generated files into
    output_dir: PathBuf,
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

    pub fn add_wrapper_pool(&mut self, original_method: &str, return_type: &str, write_read: WriteRead) {
        self.lines.push(format!("{} func gen{}(pool: DatabasePool) throws {}{{\n", self.modifier, original_method, return_type));
        self.lines.push(format!("try pool.{} {{ database in\ntry gen{}(db: database)\n}}\n}}", match write_read {
            WriteRead::Write => "write",
            WriteRead::Read => "read"
        }, original_method));
    }

    pub fn write_to_file(self, file_name: &str) {
        let mut file = File::create(self.output_dir.join(format!("{}.swift", file_name))).unwrap();

        for line in self.lines {
            writeln!(file, "{}", line).unwrap();
        }
    }
}
