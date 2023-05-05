use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn crate_mover_9000(stacks: &mut Vec<Vec<char>>, num: i32, from: usize, to: usize) {
    for _ in 0..num {
        let n = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(n);
    }
}

#[allow(dead_code)]
fn crate_mover_9001(stacks: &mut Vec<Vec<char>>, num: i32, from: usize, to: usize) {
    let mut tmp = vec![];
    for _ in 0..num {
        let n = stacks[from - 1].pop().unwrap();
        tmp.push(n);
    }
    for &c in tmp.iter().rev() {
        stacks[to - 1].push(c);
    }
}

fn show(stacks: &Vec<Vec<char>>) {
    stacks.iter().for_each(|stack| {
        print!("{}", stack.last().unwrap());
    });
    println!();
}

fn main() {
    let file = File::open("input_05.txt").unwrap();
    let buf = BufReader::new(file);

    // let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    let mut stacks = vec![
        vec!['N', 'S', 'D', 'C', 'V', 'Q', 'T'],
        vec!['M', 'F', 'V'],
        vec!['F', 'Q', 'W', 'D', 'P', 'N', 'H', 'M'],
        vec!['D', 'Q', 'R', 'T', 'F'],
        vec!['R', 'F', 'M', 'N', 'Q', 'H', 'V', 'B'],
        vec!['C', 'F', 'G', 'N', 'P', 'W', 'Q'],
        vec!['W', 'F', 'R', 'L', 'C', 'T'],
        vec!['T', 'Z', 'N', 'S'],
        vec!['M', 'S', 'D', 'J', 'R', 'Q', 'H', 'N'],
    ];

    for line in buf.lines() {
        if let Ok(s) = line {
            let v = s.split(' ').collect::<Vec<_>>();
            let num: i32 = v[1].parse().unwrap();
            let from: usize = v[3].parse().unwrap();
            let to: usize = v[5].parse().unwrap();
            // println!("{} {} {}", num, from, to);
            // crate_mover_9000(&mut stacks, num, from, to);
            crate_mover_9001(&mut stacks, num, from, to);
            // println!("{:?}", stacks)
        }
    }
    show(&stacks);
}
