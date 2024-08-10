// a doubly linked list is a list that has a pointer to the next and previous element in the list

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<Node>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Option<Link>,
    prev: Option<Link>,
}

#[derive(Clone)]
pub struct BetterTransactionLog {
    head: Option<Link>,
    tail: Option<Link>,
    length: u64,
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
    println!("Hello, world!");
}
