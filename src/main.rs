use std::env;
use std::fs;
use std::path::Path;

fn get_file_paths(path: &Path) -> std::io::Result<Vec<String>> {
    let mut file_paths: Vec<String> = Vec::new();

    // Reference: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let current_file_path = entry.path();
        file_paths.push(current_file_path.display().to_string());
    }

    Ok(file_paths)
}

fn main() -> std::io::Result<()> {
    // Reference: https://doc.rust-lang.org/std/env/fn.current_dir.html
    let current_directory = env::current_dir()?;
    println!("{}", current_directory.display());

    // Reference: https://doc.rust-lang.org/std/path/struct.PathBuf.html#examples-3
    let current_directory_path = current_directory.as_path();
    let file_paths = get_file_paths(current_directory_path)?;

    for file_path in file_paths {
        println!("{}", file_path);
    }

    Ok(())
}
