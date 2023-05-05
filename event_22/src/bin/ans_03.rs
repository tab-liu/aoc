use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn num(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - b'a' as i32 + 1,
        'A'..='Z' => c as i32 - b'A' as i32 + 26 + 1,
        _ => unreachable!(),
    }
}

fn get_a(s: &str) -> i32 {
    let mut item = '0';
    let mut map = HashMap::new();

    for c in s[..s.len() / 2].chars() {
        map.entry(c).or_insert(1);
    }

    for c in s[s.len() / 2..].chars() {
        if map.contains_key(&c) {
            item = c;
            break;
        }
    }
    num(item)
}

fn get_b(v: &Vec<String>) -> i32 {
    let mut ans = '0';
    let mut map = HashMap::new();
    for c in v[0].chars() {
        map.entry(c).or_insert(1);
    }
    for c in v[1].chars() {
        map.entry(c).and_modify(|e| *e |= 2).or_insert(2);
    }
    for c in v[2].chars() {
        if let Some(&n) = map.get(&c) {
            if n == 3 {
                ans = c;
                break;
            }
        }
    }
    num(ans)
}

fn main() {
    let file = File::open("input_03.txt").unwrap();
    let buf = BufReader::new(file);

    let mut ans_a = 0;
    let mut ans_b = 0;

    let mut v = vec![];

    let mut cnt = 0;
    for line in buf.lines() {
        if let Ok(s) = line {
            // println!("{:?}", s);
            ans_a += get_a(&s);
            v.push(s.clone());
        }
        cnt += 1;
        if cnt == 3 {
            ans_b += get_b(&v);
            cnt = 0;
            v.clear();
        }
    }
    println!("cnt: {}", cnt);
    println!("ans_a: {}", ans_a);
    println!("ans_b: {}", ans_b);
}
