// skip lists are a data structure that allow for fast search, insert, and delete operations

use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
pub struct Node {
    pub next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

pub struct SkipList {
    pub head: Link,
    pub tails: Vec<Link>,
    pub max_level: usize,
    pub length: usize,
}

// A time associated with a value
// To be able to quickly jump to an arbitary time
// To start iterating from that time

impl SkipList {
    pub fn get_level(&self) -> usize {
        let mut level = 1;
        while rand::random::<bool>() && level < self.max_level {
            level += 1;
        }
        level
    }

    pub fn append(&mut self, offset: u64, command: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        // Ensure level does not exceed max_level
        let level = level.min(self.max_level);

        let new_node = Rc::new(RefCell::new(Node {
            next: vec![None; level],
            offset,
            command,
        }));

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new_node.clone());
            }
            self.tails[i] = Some(new_node.clone());
        }

        if self.head.is_none() {
            self.head = Some(new_node.clone());
        }

        self.length += 1;
    }

    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level - 1;
                let node = head.clone();
                let mut result = None;

                loop {
                    if node.borrow().next[start_level].is_none() {
                        break;
                    }
                    start_level -= 1;
                }

                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().offset <= offset => n = next.clone(),
                            _ => break,
                        };
                    }

                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_find() {
        let mut list = SkipList {
            head: None,
            tails: vec![None; 32],
            max_level: 32,
            length: 0,
        };

        list.append(1, "hello".to_string());
        list.append(2, "world".to_string());
        list.append(3, "foo".to_string());
        list.append(4, "bar".to_string());

        assert_eq!(list.find(1), Some("hello".to_string()));
        assert_eq!(list.find(2), Some("world".to_string()));
        assert_eq!(list.find(3), Some("foo".to_string()));
        assert_eq!(list.find(4), Some("bar".to_string()));
        assert_eq!(list.find(5), None);
    }
}
