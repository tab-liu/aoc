use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
struct HeightMap {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<u8>>,
    visited: Vec<Vec<bool>>,
    queue: VecDeque<(usize, usize)>,
    // step: i32,
}

impl HeightMap {
    fn new() -> Self {
        Default::default()
    }

    fn visit(&mut self, pos: (usize, usize)) -> bool {
        if pos == self.end {
            return true;
        }
        if self.visited[pos.0][pos.1] {
            return false;
        }
        self.visited[pos.0][pos.1] = true;
        self.queue.push_back((pos.0, pos.1));
        false
    }

    fn suitable(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let height_to = self.map[to.0][to.1] as i16;
        let height_from = self.map[from.0][from.1] as i16;

        if height_to - height_from > 1 {
            false
        } else {
            true
        }
    }

    fn bfs(&mut self) -> bool {
        let row_max: usize = self.map.len() - 1;
        let col_max: usize = self.map[0].len() - 1;
        for _ in 0..self.queue.len() {
            let mut top = false;
            let pos = self.queue.pop_front().unwrap();

            if pos.0 > 0 && self.suitable(pos, (pos.0 - 1, pos.1)) {
                top |= self.visit((pos.0 - 1, pos.1));
            }

            if pos.0 < row_max && self.suitable(pos, (pos.0 + 1, pos.1)) {
                top |= self.visit((pos.0 + 1, pos.1));
            }

            if pos.1 > 0 && self.suitable(pos, (pos.0, pos.1 - 1)) {
                top |= self.visit((pos.0, pos.1 - 1));
            }

            if pos.1 < col_max && self.suitable(pos, (pos.0, pos.1 + 1)) {
                top |= self.visit((pos.0, pos.1 + 1))
            }
            if top {
                return true;
            }
        }
        false
    }

    fn climb(&mut self) -> i32 {
        let mut step = 0;
        loop {
            if self.bfs() {
                break;
            }
            step += 1;
        }
        step + 1
    }
}

fn parse(s: String) -> HeightMap {
    let mut height_map = HeightMap::new();
    for (row, line) in s.lines().enumerate() {
        if let Some(col) = line.find('S') {
            height_map.start = (row, col);
        }
        if let Some(col) = line.find('E') {
            height_map.end = (row, col);
        }
        height_map.map.push(line.chars().map(|c| c as u8).collect());
        height_map.visited.push(vec![false; line.len()]);
    }
    height_map.map[height_map.start.0][height_map.start.1] = b'a';
    height_map.map[height_map.end.0][height_map.end.1] = b'z';
    height_map
}

fn part_one(mut height_map: HeightMap) -> i32 {
    height_map.visit(height_map.start);
    height_map.climb()
}

fn part_two(mut height_map: HeightMap) -> i32 {
    for i in 0..height_map.map.len() {
        for j in 0..height_map.map[0].len() {
            if height_map.map[i][j] == b'a' {
                height_map.visit((i, j));
            }
        }
    }
    height_map.climb()
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/input_12.txt");

    let content = fs::read_to_string(path).unwrap();

    println!("{}", part_one(parse(content.clone())));
    println!("{}", part_two(parse(content)));
}
