use std::fs;
use std::path::Path;

pub fn get(path: &Path) -> std::io::Result<Vec<String>> {
    let mut file_paths: Vec<String> = Vec::new();

    // Reference: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let current_file_path = entry.path();
        file_paths.push(current_file_path.display().to_string());
    }

    Ok(file_paths)
}
