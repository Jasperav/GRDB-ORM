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
            .expect(&format!(
                "Problem formatting code in: {:#?}",
                safe_output_dir
            ));

        if !output.status.success() {
            panic!(
                "Something went wrong: {}",
                String::from_utf8(output.stderr).unwrap()
            )
        }
    } else {
        println!("Not formatting Swift code due to configuration");
    }

    if config.use_swiftlint {
        println!("Autocorrecting Swift code with Swiftlint");

        let output = Command::new("swiftlint")
            .current_dir(safe_output_dir)
            .args(&["--fix"])
            .output()
            .unwrap();

        if !output.status.success() {
            panic!(
                "Something went wrong while using Swiftlint at path '{:#?}': {}",
                safe_output_dir,
                String::from_utf8(output.stderr).unwrap()
            )
        }
    } else {
        println!("Not autocorrecting Swift code due to configuration");
    }
}
