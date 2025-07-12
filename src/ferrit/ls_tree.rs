use std::fs;
use std::io::Read;
use std::path::Path;

use flate2::read::ZlibDecoder;

/// Parse and print the contents of a tree object.
pub fn ls_tree(sha: &str, name_only: bool) {
    let (dir, file) = sha.split_at(2);
    let path = Path::new(".git/objects").join(dir).join(file);

    let compressed = fs::read(path).expect("Failed to read tree object");
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .expect("Failed to decompress object");

    // Skip "tree <size>\0"
    let null_index = decompressed.iter().position(|&b| b == 0).unwrap();
    let entries = &decompressed[null_index + 1..];

    let mut i = 0;
    while i < entries.len() {
        // Parse mode
        let mode_start = i;
        let mode_end = entries[i..].iter().position(|&b| b == b' ').unwrap();
        let mode = std::str::from_utf8(&entries[mode_start..mode_start + mode_end]).unwrap();
        i += mode_end + 1;

        // Parse name
        let name_end = entries[i..].iter().position(|&b| b == 0).unwrap();
        let name = std::str::from_utf8(&entries[i..i + name_end]).unwrap();
        i += name_end + 1;

        // Parse SHA-1 (20 bytes)
        let sha_bin = &entries[i..i + 20];
        let sha_hex = sha_bin
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        i += 20;

        if name_only {
            println!("{}", name);
        } else {
            let object_type = if mode == "40000" || mode == "040000" {
                "tree"
            } else {
                "blob"
            };
            println!("{:06} {} {}\t{}", mode, object_type, sha_hex, name);
        }
    }
}
