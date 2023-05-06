use std::fs;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/test.txt");

    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        println!("{}", line);
    }
}
