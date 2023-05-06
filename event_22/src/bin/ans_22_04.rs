use std::io::{BufRead, BufReader};
use std::{fs::File, path::PathBuf};

fn fun_a(v: Vec<&str>) -> i32 {
    let a: i32 = v[0].parse().unwrap();
    let b: i32 = v[1].parse().unwrap();
    let c: i32 = v[2].parse().unwrap();
    let d: i32 = v[3].parse().unwrap();

    if (a <= c && b >= d) || (a >= c && b <= d) {
        1
    } else {
        0
    }
}

fn fun_b(v: Vec<&str>) -> i32 {
    let a: i32 = v[0].parse().unwrap();
    let b: i32 = v[1].parse().unwrap();
    let c: i32 = v[2].parse().unwrap();
    let d: i32 = v[3].parse().unwrap();

    if c > b || a > d {
        0
    } else {
        1
    }
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/input_04.txt");

    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);

    let mut ans_a = 0;
    let mut ans_b = 0;

    for line in buf.lines() {
        if let Ok(s) = line {
            let v = s.split(&['-', ','][..]).collect::<Vec<_>>();
            ans_a += fun_a(v.clone());
            ans_b += fun_b(v);
        }
    }
    println!("ans_a: {}", ans_a);
    println!("ans_b: {}", ans_b);
}
