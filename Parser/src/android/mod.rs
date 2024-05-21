pub mod kotlin;

pub use kotlin::*;
use std::path::Path;
mod entities;

static SUPPRESS_ALL: &str =
    "@file:Suppress(\"warnings\", \"ALL\", \"UNUSED_PARAMETER\", \"RedundantSuppression\")";

pub fn generate_kotlin_package(path: &Path) -> String {
    let mut package_parts = vec![];
    let mut start_collecting = false;

    for component in path.components() {
        if let Some(component_str) = component.as_os_str().to_str() {
            if component_str == "com" {
                start_collecting = true;
            }

            // If we are collecting, push this component into the package parts
            if start_collecting {
                package_parts.push(component_str);
            }
        }
    }

    if !start_collecting {
        panic!("A com part is expected, original: {:#?}", path.components());
    }

    format!("package {}\n", package_parts.join("."))
}
