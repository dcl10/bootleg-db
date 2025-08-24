const MIN_KEYS: usize = 2;
const MAX_KEYS: usize = 4;

type Key = usize;
type Value = String;

#[derive(Debug)]
struct InternalNode {
    keys: Vec<Key>,
    values: Vec<Value>,
}

#[derive(Debug)]
struct LeafNode {
    keys: Vec<Key>,
    values: Vec<Value>,
    next: Option<Box<LeafNode>>,
}

#[derive(Debug)]
enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Node::Leaf(LeafNode {
                keys: vec![],
                values: vec![],
                next: None,
            }),
        }
    }
}
