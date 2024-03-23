#![feature(test)]

extern crate test;

pub mod linked_list;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use self::linked_list::List;

    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[bench]
    fn bench_list_append(b: &mut Bencher) {
        let mut list = List::new_empty();
        b.iter(|| {
            for i in 0..1000 {
                list.append(i);
            }
        });
    }

    #[test]
    fn test_list_new_empty() {
        let mut list: List<i32> = List::new_empty();
        assert_eq!(list.length, 0);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_append() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
    }

    #[test]
    fn test_list_pop() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.length, 0);
        assert_eq!(list.pop(), None);
    }
}
