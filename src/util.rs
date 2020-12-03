use std::fs;
use std::path::PathBuf;

pub fn read_input(path: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("inputs");
    d.push(path);

    fs::read_to_string(d).expect("Failed to read file!")
}
