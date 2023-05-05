use std::fs::File;
use std::io::{BufRead, BufReader};

fn score_a(a: &str, b: &str) -> i32 {
    let win = [3, 0, 6, 3, 0, 6];
    let s_a = match a {
        "A" => 4,
        "B" => 5,
        _ => 6,
    };
    let s_b = match b {
        "X" => 1,
        "Y" => 2,
        _ => 3,
    };

    s_b + win[(s_a - s_b) as usize]
}

fn score_b(a: &str, b: &str) -> i32 {
    let s_a = match a {
        "A" => 1,
        "B" => 2,
        _ => 3,
    };
    let ans = match b {
        "X" => {
            if s_a == 1 {
                3
            } else {
                s_a - 1
            }
        }
        "Y" => s_a + 3,
        _ => {
            if s_a == 2 {
                9
            } else {
                (s_a + 1) % 3 + 6
            }
        }
    };
    // println!("a: {}, {}", s_a, ans);
    ans
}

fn main() {
    let file = File::open("input_02.txt").unwrap();
    let buf = BufReader::new(file);

    let mut ans_a = 0;
    let mut ans_b = 0;

    for line in buf.lines() {
        if let Ok(s) = line {
            let v: Vec<&str> = s.split(' ').collect();
            // println!("{:?}", v);
            ans_a += score_a(v[0], v[1]);
            ans_b += score_b(v[0], v[1]);
        }
    }
    println!("ans_a: {}", ans_a);
    println!("ans_b: {}", ans_b);
}
