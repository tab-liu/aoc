use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input_01.txt").unwrap();
    let buf = BufReader::new(file);

    let mut max = (0, 0, 0);
    let mut elf = vec![];

    for line in buf.lines() {
        if let Ok(n) = line {
            if n.len() > 0 {
                // println!("{}", n);
                elf.push(n.parse::<i32>().unwrap());
            } else {
                let tmp = elf.iter().sum();
                if tmp > max.0 {
                    max = (tmp, max.0, max.1);
                } else if tmp > max.1 {
                    max = (max.0, tmp, max.1);
                } else if tmp > max.2 {
                    max = (max.0, max.1, tmp);
                }
                // println!("max: {}", max);
                elf.clear();
            }
        }
    }
    println!("get max: {:?}", max);
    println!("get sum: {}", max.0 + max.1 + max.2);
}
