use std::fs;
use std::io::Read;
use std::path::Path;

use flate2::read::ZlibDecoder;

pub fn cat_file(hash: &str) {
    if hash.len() != 40 {
        eprintln!("Invalid object hash length");
        return;
    }

    // construct the full path to the Git object in .git/objects/<dir>/<file>
    let (dir, file) = hash.split_at(2);
    let path = Path::new(".git/objects").join(dir).join(file);

    let file = fs::File::open(&path).expect("Object not found");

    // decorder will give me the decompressed bytes when I read from it.
    let mut decoder = ZlibDecoder::new(file);
    let mut decoded = Vec::new();
    decoder
        .read_to_end(&mut decoded)
        .expect("Decompression failed");

    // Git object format: "<type> <size>\0<content>"
    // iterates through decoded and finds the first null value and return its position
    if let Some(null_index) = decoded.iter().position(|&b| b == 0) {
        //  _header will contain type, content will contain file text
        let (_header, content) = decoded.split_at(null_index + 1);
        print!("{}", String::from_utf8_lossy(content));
    } else {
        eprintln!("Invalid object format");
    }
}
