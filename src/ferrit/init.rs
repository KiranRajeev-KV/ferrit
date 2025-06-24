use std::fs;
use std::io::Write;
use std::path::Path;

pub fn init() {
    let dir = Path::new(".git");

    if dir.exists() {
        println!("Repository already exists.");
        return;
    }

    // Create directory structure
    fs::create_dir(dir).expect("Failed to create .git directory");
    fs::create_dir_all(dir.join("objects")).unwrap();
    fs::create_dir_all(dir.join("refs").join("heads")).unwrap();

    // Create .git/HEAD file
    let head_path = dir.join("HEAD");
    let mut head_file = fs::File::create(&head_path).expect("Failed to create .git/HEAD file");
    writeln!(head_file, "ref: refs/heads/main").unwrap();

    println!("Initialized empty git repository");
}
