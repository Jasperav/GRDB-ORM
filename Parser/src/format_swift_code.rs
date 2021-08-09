use crate::configuration::Config;
use std::path::Path;
use std::process::Command;

/// Formats the generated code if needed
pub fn format_swift_code(config: &Config, safe_output_dir: &Path) {
    if config.use_swiftformat {
        println!("Formatting Swift code with Swiftformat");

        let output = Command::new("swiftformat")
            .current_dir(safe_output_dir)
            // TODO Not sure how the --swiftversion flag works, can't get it to work
            .args(&["."])
            .output()
            .unwrap();

        if !output.status.success() {
            panic!("{}", String::from_utf8(output.stderr).unwrap())
        }
    } else {
        println!("Not formatting Swift code due to configuration");
    }

    if config.use_swiftlint {
        println!("Autocorrecting Swift code with Swiftlint");

        let output = Command::new("swiftlint")
            .current_dir(safe_output_dir)
            .args(&["autocorrect"])
            .output()
            .unwrap();

        if !output.status.success() {
            panic!("{}", String::from_utf8(output.stderr).unwrap())
        }
    } else {
        println!("Not autocorrecting Swift code due to configuration");
    }
}
