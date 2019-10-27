use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    // Reference: https://doc.rust-lang.org/std/env/fn.current_dir.html
    let current_directory = env::current_dir()?;
    println!("{}", current_directory.display());

    // Reference: https://doc.rust-lang.org/std/path/struct.PathBuf.html#examples-3
    let current_directory_path = current_directory.as_path();

    // Reference: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    for entry in fs::read_dir(current_directory_path)? {
        let entry = entry?;
        let current_file_path = entry.path();
        println!("{}", current_file_path.display());
    }

    Ok(())
}
