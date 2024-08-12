// Binary Search Tree is a node-based binary tree data structure which has the following properties:

type Tree = Option<Box<Node>>;

pub struct Node {
    pub value: u64,
    left: Tree,
    right: Tree,
}

pub struct BinarySearchTree {
    root: Tree,
    pub length: u64,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, value: u64) {
        // Add a new node to the tree
        self.length += 1;
        let new_node = Node {
            value,
            left: None,
            right: None,
        };
        let new_node = Some(Box::new(new_node));

        // If the tree is empty, set the new node as the root
        match self.root {
            None => {
                self.root = new_node;
            }
            Some(ref mut node) => {
                BinarySearchTree::add_node(node, new_node);
            }
        }
    }

    fn add_node(node: &mut Node, new_node: Tree) {
        // Add a new node to the tree
        if new_node.as_ref().unwrap().value < node.value {
            match node.left {
                None => {
                    node.left = new_node;
                }
                Some(ref mut left_node) => {
                    BinarySearchTree::add_node(left_node, new_node);
                }
            }
        } else {
            match node.right {
                None => {
                    node.right = new_node;
                }
                Some(ref mut right_node) => {
                    BinarySearchTree::add_node(right_node, new_node);
                }
            }
        }
    }

    // find
    pub fn find(&self, value: u64) -> bool {
        // Find a node in the tree
        match self.root {
            None => false,
            Some(ref node) => BinarySearchTree::find_node(node, value),
        }
    }

    pub fn find_node(node: &Node, value: u64) -> bool {
        // Find a node in the tree
        if value == node.value {
            true
        } else if value < node.value {
            match node.left {
                None => false,
                Some(ref left_node) => BinarySearchTree::find_node(left_node, value),
            }
        } else {
            match node.right {
                None => false,
                Some(ref right_node) => BinarySearchTree::find_node(right_node, value),
            }
        }
    }

    pub fn find_right(&self, value: u64) -> Option<u64> {
        // Find the right node in the tree
        match self.root {
            None => None,
            Some(ref node) => BinarySearchTree::find_right_node(node, value),
        }
    }

    pub fn find_right_node(node: &Node, value: u64) -> Option<u64> {
        // Find the right node in the tree
        if value == node.value {
            match node.right {
                None => None,
                Some(ref right_node) => Some(right_node.value),
            }
        } else if value < node.value {
            match node.left {
                None => None,
                Some(ref left_node) => BinarySearchTree::find_right_node(left_node, value),
            }
        } else {
            match node.right {
                None => None,
                Some(ref right_node) => BinarySearchTree::find_right_node(right_node, value),
            }
        }
    }

    // visualize the tree
    pub fn visualize(&self) {
        // Visualize the tree
        match self.root {
            None => println!("Empty tree"),
            Some(ref node) => BinarySearchTree::visualize_node(node, 0),
        }
    }

    pub fn visualize_node(node: &Node, level: u64) {
        // Visualize the tree
        if let Some(ref right_node) = node.right {
            BinarySearchTree::visualize_node(right_node, level + 1);
        }

        println!("{:width$}{}", "", node.value, width = (level * 4) as usize);

        if let Some(ref left_node) = node.left {
            BinarySearchTree::visualize_node(left_node, level + 1);
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::new();

    tree.add(5);
    tree.add(3);
    tree.add(7);

    println!("{}", tree.find(3)); // true
    println!("{}", tree.find(9)); // true

    println!("{:?}", tree.find_right(3)); // Some(5)
    println!("{:?}", tree.find_right(5)); // Some(7)

    tree.visualize();
}
