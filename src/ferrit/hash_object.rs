use std::fs;
use std::io::Write;
use std::path::Path;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

pub fn hash_object(filename: &str, write: bool) -> String {
    let data = fs::read(filename).expect("Failed to read file");

    // Prepare the Git blob object format
    let header = format!("blob {}\0", data.len());
    let mut full = header.into_bytes();
    full.extend_from_slice(&data);

    // Compute SHA-1 hash
    let hash = Sha1::digest(&full);
    let hex = format!("{:x}", hash);

    if write {
        let (dir, file) = hex.split_at(2);
        let obj_dir = Path::new(".git/objects").join(dir);
        let obj_path = obj_dir.join(file);

        fs::create_dir_all(&obj_dir).expect("Failed to create object directory");

        let file = fs::File::create(&obj_path).expect("Failed to create object file");
        let mut encoder = ZlibEncoder::new(file, Compression::default());
        encoder.write_all(&full).expect("Failed to compress object");
        encoder.finish().expect("Failed to finish writing object");
    }

    hex
}
