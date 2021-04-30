use std::path::{Path, PathBuf};

/// Initializes the output dir
pub fn initialize(output_dir: &Path) -> PathBuf {
    println!("Initializing output dir");
    // Append 'generated' as suffix to make deletion of dir more safe
    let safe_output_dir = create_safe_dir(output_dir);

    // First empty the output dir, don't handle any error (since the folder can be non-existent)
    let _ = std::fs::remove_dir_all(&safe_output_dir);

    // Create the folder to put the generated files in
    std::fs::create_dir_all(&safe_output_dir).unwrap();

    safe_output_dir
}

pub fn create_safe_dir(path: &Path) -> PathBuf {
    path.join("generated")
}
