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

    pub fn search(&self, key: Key) -> Option<&Value> {
        match &self.root {
            Node::Internal(_) => None, // None for now
            Node::Leaf(leaf) => {
                for (i, k) in leaf.keys.iter().enumerate() {
                    if *k == key {
                        return Some(&leaf.values[i]);
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_existing_key() {
        let leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["hello".to_string(), "world!".to_string()],
            next: None,
        };
        let tree = Tree {
            root: Node::Leaf(leaf),
        };

        let result = tree.search(2);
        assert_eq!(result.map(|s| s.as_str()), Some("world!"));
    }

    #[test]
    fn test_search_nonexistent_key() {
        let leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["hello".to_string(), "world!".to_string()],
            next: None,
        };
        let tree = Tree {
            root: Node::Leaf(leaf),
        };

        let result = tree.search(3);
        assert_eq!(result, None);
    }
}
