use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
struct CPU {
    cycle: i32,
    preparing: i32,
    ready: i32,
    reg: i32,
    strength: i32,
}

impl CPU {
    fn new() -> Self {
        CPU {
            reg: 1,
            ..Default::default()
        }
    }

    fn tick(&mut self, val: i32) {
        self.cycle += 1;
        self.reg += self.ready;
        self.ready = self.preparing;
        self.preparing = val;
        if (self.cycle + 20) % 40 == 0 {
            self.strength += self.cycle * self.reg;
        }
        let pos = (self.cycle - 1) % 40;
        let c = if pos < self.reg - 1 || pos > self.reg + 1 {
            '.'
        } else {
            '#'
        };
        print!("{c}");
        if pos == 39 {
            println!();
        }
    }
}

fn parse(s: &str) -> i32 {
    if s.starts_with("noop") {
        0
    } else {
        let mut iter = s.split_whitespace();
        iter.next();
        iter.next().unwrap().parse().unwrap()
    }
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/input_10.txt");

    let mut cpu = CPU::new();

    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        let val = parse(line);
        cpu.tick(val);
        if val != 0 {
            cpu.tick(0);
        }
    }
    println!("{}", cpu.strength);
}
