use std::env;

mod file_paths;

fn main() -> std::io::Result<()> {
    // Reference: https://doc.rust-lang.org/std/env/fn.current_dir.html
    let current_directory = env::current_dir()?;
    println!("{}", current_directory.display());

    // Reference: https://doc.rust-lang.org/std/path/struct.PathBuf.html#examples-3
    let current_directory_path = current_directory.as_path();
    let file_paths = file_paths::get(current_directory_path)?;

    for file_path in file_paths {
        println!("{}", file_path);
    }

    Ok(())
}
