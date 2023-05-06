use std::collections::HashMap;
use std::path::PathBuf;
#[derive(Debug)]
struct Arena {
    map: HashMap<usize, Item>,
    cur: usize,
}

#[derive(Debug)]
struct Item {
    size: usize,
    is_dir: bool,
    name: String,
    parent: Option<usize>,
    child: Vec<usize>,
}

impl Arena {
    fn new() -> Self {
        let mut map = HashMap::new();
        let root = Item {
            size: 0,
            is_dir: true,
            name: "/".to_string(),
            parent: None,
            child: Vec::new(),
        };
        map.insert(0, root);
        Arena { map, cur: 0 }
    }

    fn dir_size(&mut self) {
        let mut size = 0;
        let item = self.map.get(&self.cur).unwrap();

        for id in &item.child {
            let leafe = self.map.get(id).unwrap();
            size += leafe.size;
        }

        let item = self.map.get_mut(&self.cur).unwrap();
        item.size = size;
    }

    fn cd_dir(&mut self, name: &str) {
        if name == ".." {
            self.dir_size();
        }
        let item = self.map.get(&self.cur).unwrap();

        if name == ".." {
            if let Some(pre) = item.parent {
                self.cur = pre;
            }
            return;
        }

        for id in &item.child {
            let leafe = self.map.get(id).unwrap();
            if leafe.is_dir && leafe.name == name {
                self.cur = *id;
                return;
            }
        }
    }

    fn add_dir(&mut self, name: &str) {
        // add new item to arena
        let id = self.map.len(); // new id
        let new_item = Item {
            size: 0,
            is_dir: true,
            name: name.to_string(),
            parent: Some(self.cur),
            child: Vec::new(),
        };
        self.map.insert(id, new_item);

        // add id to cur item
        if let Some(item) = self.map.get_mut(&self.cur) {
            item.child.push(id);
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        // add new item to arena
        let id = self.map.len(); // new id
        let new_item = Item {
            size,
            is_dir: false,
            name: name.to_string(),
            parent: Some(self.cur),
            child: Vec::new(),
        };
        self.map.insert(id, new_item);

        // add id to cur item
        if let Some(item) = self.map.get_mut(&self.cur) {
            item.child.push(id);
        }
    }
}

fn cmd_parser(arena: &mut Arena, s: &str) {
    // println!("{s}");
    let mut iter = s.split_whitespace().skip(1);
    let cmd = iter.next().unwrap();
    if cmd == "ls" {
        return;
    }
    let name = iter.next().unwrap();
    // println!("{cmd} {dir}")
    arena.cd_dir(name);
}

fn dir_parser(arena: &mut Arena, s: &str) {
    let name = s.split_whitespace().skip(1).next().unwrap();
    arena.add_dir(name);
}

fn file_parser(arena: &mut Arena, s: &str) {
    let mut iter = s.split_whitespace();
    let size = iter.next().unwrap().parse().unwrap();
    let name = iter.next().unwrap();
    arena.add_file(name, size);
}

fn go_to_root(arena: &mut Arena) {
    while arena.cur > 0 {
        arena.cd_dir("..");
    }
    arena.dir_size();
}

fn walk_a(arena: &Arena) -> usize {
    arena
        .map
        .iter()
        .filter_map(|(_, item)| {
            if item.is_dir && item.size <= 100_000 {
                Some(item.size)
            } else {
                None
            }
        })
        .sum()
}

fn walk_b(arena: &Arena) -> usize {
    let used_space = arena.map.get(&0).unwrap().size;
    let left_space = 70_000_000 - used_space;
    let need_free = 30_000_000 - left_space;
    arena
        .map
        .iter()
        .filter_map(|(_, item)| {
            if item.is_dir && item.size >= need_free {
                Some(item.size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let mut arena = Arena::new();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/input_07.txt");

    let contents = std::fs::read_to_string(path).unwrap();
    for line in contents.lines().skip(1) {
        // println!("{line}");
        if line.starts_with("$") {
            cmd_parser(&mut arena, line);
        } else if line.starts_with("dir") {
            dir_parser(&mut arena, line);
        } else {
            file_parser(&mut arena, line);
        }
    }
    go_to_root(&mut arena); // calc all dir size
                            // println!("{:#?}", arena);
    println!("part one: {}", walk_a(&arena));
    println!("part two: {}", walk_b(&arena));
}
