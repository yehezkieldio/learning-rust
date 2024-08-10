// a doubly linked list is a list that has a pointer to the next and previous element in the list

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<Node>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Option<Link>,
    prev: Option<Link>,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Clone)]
pub struct BetterTransactionLog {
    head: Option<Link>,
    tail: Option<Link>,
    length: u64,
}

impl BetterTransactionLog {
    pub fn new() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => {
                self.head = Some(new.clone());
            }
        }

        self.length += 1;
        self.tail = Some(new);
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }

    pub fn iter_back(&self) -> ListIterator {
        ListIterator::new(self.tail.clone())
    }

    pub fn len(&self) -> u64 {
        self.length
    }
}

// Moves forward through the log
// Moves backward through the log
// Moves don't consume the log

pub struct ListIterator {
    current: Option<Link>,
}

impl ListIterator {
    fn new(start_at: Option<Link>) -> ListIterator {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // Borrow the current node
        let current = &self.current;
        // Initialize the result
        let mut result = None;

        // Update the current node
        self.current = match current {
            // If the current node is Some, borrow the node and update the result
            Some(ref current) => {
                // Borrow the node
                let current = current.borrow();
                // Update the result
                result = Some(current.value.clone());
                // Move to the next node
                current.next.clone()
            }
            // If the current node is None, return None
            None => None,
        };

        // Return the result
        result
    }
}

// A double-ended iterator is a type of iterator that can move both forwards and backwards through a sequence
// The difference between a double-ended iterator and a regular iterator is that a double-ended iterator has a next_back method
// The next_back method moves the iterator backwards through the sequence
impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                // Move to the previous node
                current.prev.clone()
            }
            None => None,
        };

        result
    }
}

fn main() {
    let logs = vec!["hello", "world", "foo", "bar"];
    let mut log = BetterTransactionLog::new();

    for entry in logs {
        log.append(entry.to_string());
    }

    for entry in log.iter() {
        println!("{}", entry);
    }

    for entry in log.iter_back() {
        println!("{}", entry);
    }

    println!("Length: {}", log.len());
}
