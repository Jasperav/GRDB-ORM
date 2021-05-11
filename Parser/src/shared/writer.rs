use crate::configuration::Config;

use crate::shared::data_extensions::write_data_extensions;
use crate::shared::json_codable::write_json_coder;

/// Writes the [Shared] [Swift.Enum] type
pub fn write(config: &Config) {
    println!("Writing the Shared enum type");

    let mut line_writer = config.create_line_writer();

    line_writer.add_line("enum Shared {".to_string());

    write_json_coder(&mut line_writer);

    line_writer.add_closing_brackets();

    write_data_extensions(&mut line_writer);

    line_writer.write_to_file("Shared");
}
