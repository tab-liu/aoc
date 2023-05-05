struct Node {
    children: Vec<Box<Node>>,
    parent: Option<Box<Node>>, // add parent field
                               // add more fields here
}

fn main() {
    let mut root = Node {
        children: vec![],
        parent: None, // parent of the root is None
                      // add more fields here
    };

    let mut child1 = Node {
        children: vec![],
        parent: Some(Box::new(root)), // set parent to root
                                      // add more fields here
    };

    root.children.push(Box::new(child1)); // add child to root's children
}
