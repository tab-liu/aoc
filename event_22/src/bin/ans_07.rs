use std::collections::HashMap;

#[derive(Debug)]
enum FileType {
    Dir,
    File,
}

#[derive(Debug)]
struct Node {
    size: usize,
    file_type: FileType,
    name: String,
    items: Vec<Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            size: 0,
            file_type: FileType::Dir,
            name: "/".into(),
            items: Vec::new(),
        }
    }

    fn add_dir(&mut self, name: &str) {
        self.items.push(Node {
            size: 0,
            file_type: FileType::Dir,
            name: name.into(),
            items: Vec::new(),
        });
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.items.push(Node {
            size,
            file_type: FileType::File,
            name: name.into(),
            items: Vec::new(),
        });
    }
}

struct Tree {
    stack: Vec<Box<Node>>,
    node: Node,
    dirs: HashMap<String, usize>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            stack: vec![],
            node: Node::new(),
        }
    }
}

fn main() {
    let mut node = Node::new();
    let lines = include_str!("test.txt").lines();
    for line in lines {
        // println!("{line}");
        if line.starts_with("$") {
            println!("cmd---> {}", line);
        } else if line.starts_with("dir") {
            println!("dir || {}", line);
        } else {
            println!("file ... {}", line);
        }
    }
}
