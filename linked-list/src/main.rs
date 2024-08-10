// A singly linked list implementation using Rc<RefCell<Node>> to allow multiple ownership and interior mutability.

use std::{cell::RefCell, rc::Rc};

type SingleLink = Option<Rc<RefCell<Node>>>;

// Node struct is designed to hold a value and a reference to the next node in the list.
#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

// The new method creates  a new Node wrapped in Rc<RefCell<Node>> to allow multiple ownership and interior mutability.
impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }

    pub fn peek(&self) -> Option<String> {
        self.head.as_ref().map(|node| node.borrow().value.clone())
    }

    pub fn get_all(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            result.push(node.borrow().value.clone());
            current = node.borrow().next.clone();
        }
        result
    }
}

fn main() {
    let mut log = TransactionLog::new_empty();
    log.append("Hello".to_string());
    log.append("World".to_string());
    log.append("Rust".to_string());

    let all = log.get_all();
    println!("All: {:?}", all);

    log.pop();
    print!("After pop: ");

    let all = log.get_all();
    println!("All: {:?}", all);

    let value = log.peek();
    println!("Value: {:?}", value);
    println!("Length: {:?}", log.length);
}
