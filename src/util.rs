use std::path::PathBuf;
use std::fs;

pub fn read_input(path: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("inputs");
    d.push(path);

    return fs::read_to_string(d).expect(&format!("Failed to read file!"));
}
