use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("test.txt").unwrap();
    let buf = BufReader::new(file);

    for line in buf.lines() {
        if let Ok(n) = line {
            println!("{}", n);
        }
    }
}
