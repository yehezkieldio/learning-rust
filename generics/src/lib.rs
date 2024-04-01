use std::{cmp, ops::Index};

const MIN_SIZE: usize = 10;

type Node<T> = Option<T>;

pub struct DynamicArray<T>
where
    T: Sized + Clone,
{
    buf: Box<[Node<T>]>,
    cap: usize,
    pub length: usize,
}

impl<T> DynamicArray<T>
where
    T: Sized + Clone,
{
    pub fn new_array() -> DynamicArray<T> {
        DynamicArray {
            buf: vec![None; MIN_SIZE].into_boxed_slice(),
            length: 0,
            cap: MIN_SIZE,
        }
    }

    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);
        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;

        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }

    pub fn append(&mut self, value: T) {
        if self.length == self.cap {
            self.grow(self.length + 1);
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
    }

    pub fn at(&mut self, index: usize) -> Node<T> {
        if self.length > index {
            self.buf[index].clone()
        } else {
            None
        }
    }
}

impl<T> Index<usize> for DynamicArray<T>
where
    T: Sized + Clone,
{
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        if self.length > index {
            &self.buf[index]
        } else {
            &None
        }
    }
}

impl<T> Clone for DynamicArray<T>
where
    T: Sized + Clone,
{
    fn clone(&self) -> Self {
        DynamicArray {
            buf: self.buf.clone(),
            cap: self.cap,
            length: self.length,
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn dynamic_array_clone() {
        let mut list = DynamicArray::new_array();
        list.append(3.14);
        let mut list2 = list.clone();
        list2.append(42.0);
        assert_eq!(list[0], Some(3.14));
        assert_eq!(list[1], None);
        assert_eq!(list2[0], Some(3.14));
        assert_eq!(list2[1], Some(42.0));
    }

    #[test]
    fn dynamic_array_index() {
        let mut list = DynamicArray::new_array();
        list.append(3.14);

        assert_eq!(list[0], Some(3.14));
        let mut list = DynamicArray::new_array();
        list.append("Hello");
        assert_eq!(list[0], Some("Hello"));
        assert_eq!(list[1], None);
    }

    #[test]
    fn dynamic_array_2d_array() {
        let mut list = DynamicArray::new_array();
        let mut sublist = DynamicArray::new_array();
        sublist.append(3.14);
        list.append(sublist);
        assert_eq!(list.at(0).unwrap().at(0), Some(3.14));
        assert_eq!(list[0].as_ref().unwrap()[0], Some(3.14));
    }
}
