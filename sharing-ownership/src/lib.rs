#![feature(test)]

extern crate test;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use test::{black_box, Bencher};

    fn length(s: String) -> usize {
        s.len()
    }

    fn rc_length(s: Rc<String>) -> usize {
        s.len()
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn cloning() {
        let s = "abcdef".to_owned();
        assert_eq!(length(s), 6);
        // s is moved here and no longer valid

        let s = "abcdef".to_owned();
        for _ in 0..10 {
            // clone is a expensive shallow copy
            // shallow copy is a copy of the reference
            assert_eq!(length(s.clone()), 6);
        }
    }

    #[bench]
    fn bench_string_clone(b: &mut Bencher) {
        let s: String = (0..100_000).map(|_| 'a').collect();
        b.iter(|| black_box(length(s.clone())))
    }

    #[test]
    fn refcounting() {
        let s = Rc::new("abcdef".to_owned());
        // we can clone Rc (reference count) with low cost
        assert_eq!(rc_length(s.clone()), 6);

        for _ in 0..10 {
            assert_eq!(rc_length(s.clone()), 6);
        }
    }

    #[bench]
    fn bench_string_rc(b: &mut Bencher) {
        let s: String = (0..100_000).map(|_| 'a').collect();
        let rc_s = Rc::new(s);
        b.iter(|| black_box(rc_length(rc_s.clone())))
    }
}
