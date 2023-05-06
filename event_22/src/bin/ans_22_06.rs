use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn end_with(s: &str, d: usize) -> usize {
    let mut ans = 0;
    let mut start = 0;
    let mut map = HashMap::new();
    for (n, c) in s.chars().enumerate() {
        if let Some(idx) = map.get_mut(&c) {
            if n - *idx <= d - 1 {
                start = start.max(*idx + 1);
            }
            *idx = n;
        } else {
            map.insert(c, n);
        }
        if n - start == d - 1 {
            ans = n + 1;
            break;
        }
    }
    println!("{}", &s[start..ans]);
    ans
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/input_06.txt");

    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);

    let mut ans = 0;
    let mut ans_2 = 0;

    for line in buf.lines() {
        if let Ok(s) = line {
            ans = end_with(&s, 4);
            ans_2 = end_with(&s, 14);
        }
        println!("{} - {}", ans, ans_2);
    }
}
