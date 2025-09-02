use std::fs;
use std::path::{Path};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::io::Write;
use std::os::unix::ffi::OsStrExt;

use crate::ferrit::hash_object::hash_object;

// Recursively build a tree object for the given directory.
fn build_tree(dir: &Path) -> String {
    let mut entries = Vec::new();

    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to get directory entry");
        let path = entry.path();
        let name = entry.file_name();
        let name_bytes = name.as_os_str().as_bytes();

        // skip `.git` directory
        if name == ".git" {
            continue;
        }

        if path.is_file() {
            // Create a blob object
            let sha = hash_object(path.to_str().unwrap(), true);
            let mode = b"100644"; // regular file

            let mut entry = Vec::new();
            entry.extend_from_slice(mode);
            entry.push(b' ');
            entry.extend_from_slice(name_bytes);
            entry.push(0);
            entry.extend_from_slice(&hex_to_bytes(&sha));
            entries.push((name.clone(), entry));
        } else if path.is_dir() {
            // Recurse into subdirectory
            let sha = build_tree(&path);
            let mode = b"40000"; // directory (tree)

            let mut entry = Vec::new();
            entry.extend_from_slice(mode);
            entry.push(b' ');
            entry.extend_from_slice(name_bytes);
            entry.push(0);
            entry.extend_from_slice(&hex_to_bytes(&sha));
            entries.push((name.clone(), entry));
        }
    }

    // Sort entries by name (Git requirement)
    entries.sort_by_key(|(name, _)| name.clone());

    // Flatten into a single byte vector
    let mut content = Vec::new();
    for (_, entry) in entries {
        content.extend_from_slice(&entry);
    }

    // Add header: "tree <size>\0"
    let header = format!("tree {}\0", content.len());
    let mut full = header.into_bytes();
    full.extend_from_slice(&content);

    // Hash the tree
    let sha = Sha1::digest(&full);
    let hex = format!("{:x}", sha);

    // Write to .git/objects
    let (dir_name, file_name) = hex.split_at(2);
    let obj_dir = Path::new(".git/objects").join(dir_name);
    let obj_path = obj_dir.join(file_name);

    fs::create_dir_all(&obj_dir).expect("Failed to create object directory");
    let file = fs::File::create(&obj_path).expect("Failed to create object file");
    let mut encoder = ZlibEncoder::new(file, Compression::default());
    encoder.write_all(&full).expect("Failed to compress object");
    encoder.finish().expect("Failed to finish writing object");

    hex
}

// Helper: convert hex SHA string -> raw 20-byte Vec<u8>
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

// Public entry point for `write-tree`
pub fn write_tree() {
    let sha = build_tree(Path::new("."));
    println!("{}", sha);
}
