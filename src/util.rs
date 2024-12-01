use std::{fs, path::Path};

pub fn read_to_string(file: &str) -> String {
    let path = Path::new(&file);
    let content = fs::read_to_string(path).expect("expected file to exist");
    return content;
}
