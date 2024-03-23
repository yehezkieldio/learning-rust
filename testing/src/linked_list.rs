use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node<T>
where
    T: Sized + Clone,
{
    value: T,
    next: Link<T>,
}

impl<T> Node<T>
where
    T: Sized + Clone,
{
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct List<T>
where
    T: Sized + Clone,
{
    head: Link<T>,
    tail: Link<T>,
    pub length: usize,
}

impl<T> List<T>
where
    T: Sized + Clone,
{
    // Create a new empty list
    pub fn new_empty() -> List<T> {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }

    // Create a new list with a single element
    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    // Remove the first element from the list and return it
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went terribly wrong!")
                .into_inner()
                .value
        })
    }
}
