#[derive(Debug)]
enum FileType {
    Dir(Vec<Node>),
    File,
}

#[derive(Debug)]
struct Node {
    size: usize,
    file_type: FileType,
    name: String,
}

struct Tree<'a> {
    stack: Vec<&'a mut Node>,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Node {
            size: 0,
            file_type: FileType::Dir,
            name: "/".into(),
            child: Vec::new(),
            parent: None,
        }
    }

    fn add_dir(&mut self, name: &str) -> &Node {
        let cur = self.clone();
        let node = Node {
            size: 0,
            file_type: FileType::Dir,
            name: name.into(),
            child: Vec::new(),
            parent: Some(self),
        };
        self.child.push(Box::new(node));
        self.child.last().unwrap()
    }

    fn add_file(&mut self, name: &str, size: usize) {
        let node = Node {
            size,
            file_type: FileType::File,
            name: name.into(),
            child: Vec::new(),
            parent: None,
        };
        self.child.push(Box::new(node));
    }

    fn dir_size(&self) -> usize {
        0
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
