use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Position {
    nodes: Vec<(i32, i32)>,
    tail_visited_1: HashSet<(i32, i32)>,
    tail_visited_9: HashSet<(i32, i32)>,
}

impl Position {
    fn new() -> Self {
        let mut set_1 = HashSet::new();
        let mut set_9 = HashSet::new();
        set_1.insert((0, 0));
        set_9.insert((0, 0));
        Position {
            nodes: vec![(0, 0); 10],
            tail_visited_1: set_1,
            tail_visited_9: set_9,
        }
    }

    fn motions(&mut self, head: usize, tail: usize) {
        if (self.nodes[head].0 - self.nodes[tail].0).abs() <= 1
            && (self.nodes[head].1 - self.nodes[tail].1).abs() <= 1
        {
            return;
        }
        let mut dx = self.nodes[head].0 - self.nodes[tail].0;
        let mut dy = self.nodes[head].1 - self.nodes[tail].1;

        if dx.abs() == 2 {
            dx /= 2;
        }
        if dy.abs() == 2 {
            dy /= 2
        }

        self.nodes[tail] = (self.nodes[tail].0 + dx, self.nodes[tail].1 + dy);

        if tail == 1 {
            self.tail_visited_1.insert(self.nodes[tail]);
        }

        if tail == 9 {
            self.tail_visited_9.insert(self.nodes[tail]);
        }
    }

    fn shift(&mut self, dir: Dir, step: i32) {
        for _ in 0..step {
            self.nodes[0] = match dir {
                Dir::Left => (self.nodes[0].0 - 1, self.nodes[0].1),
                Dir::Right => (self.nodes[0].0 + 1, self.nodes[0].1),
                Dir::Up => (self.nodes[0].0, self.nodes[0].1 - 1),
                Dir::Down => (self.nodes[0].0, self.nodes[0].1 + 1),
            };

            for i in 0..9 {
                self.motions(i, i + 1);
            }
        }
    }
}

fn parser(s: &str) -> (Dir, i32) {
    let mut iter = s.split_whitespace();
    let dir = iter.next().unwrap();
    let step = iter.next().unwrap().parse().unwrap();
    let dir = match dir {
        "L" => Dir::Left,
        "R" => Dir::Right,
        "U" => Dir::Up,
        "D" => Dir::Down,
        _ => unreachable!(),
    };
    (dir, step)
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/input_09.txt");

    let mut pos = Position::new();

    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        let (dir, step) = parser(line);
        pos.shift(dir, step);
    }
    println!("{}", pos.tail_visited_1.iter().count());
    println!("{}", pos.tail_visited_9.iter().count());
}
