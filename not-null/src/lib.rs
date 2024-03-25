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
    #[should_panic]
    fn option_unwrap() {
        assert_eq!(Some(10).unwrap(), 10);
        assert_eq!(None.unwrap_or(10), 10);
        assert_eq!(None.unwrap_or_else(|| 5 * 2), 10);

        Option::<i32>::None.unwrap();
        Option::<i32>::None.expect("Better say something when panicking!");
    }

    #[test]
    fn option_working_with_values() {
        let mut o = Some(42);

        let nr = o.take();
        assert!(o.is_none());
        assert_eq!(nr, Some(42));

        let mut o = Some(42);
        assert_eq!(o.replace(1535), Some(42));
        assert_eq!(o, Some(1535));

        let o = Some(1535);
        assert_eq!(o.map(|v| format!("{:#x}", v)), Some("0x5ff".to_owned()));

        let o = Some(1532);
        match o.ok_or("Nope") {
            Ok(nr) => assert_eq!(nr, 1532),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn option_sequentials() {
        let a = Some(42);
        let b = Some(1532);

        // and returns None if either of the Option is None, other it returns the second Option.
        assert_eq!(a.and(b), Some(1532));
        assert_eq!(a.and(Option::<i32>::None), None);
        // or returns the first Option if it is Some, otherwise it returns the second Option.
        assert_eq!(a.or(None), Some(42));
        assert_eq!(a.or(b), Some(42));
        assert_eq!(None.or(a), Some(42));

        let new_a = a.and_then(|v| Some(v + 100)).filter(|&v| v != 42);
        assert_eq!(new_a, Some(142));
        let mut a_iter = new_a.iter();
        assert_eq!(a_iter.next(), Some(&142));
        assert_eq!(a_iter.next(), None);
    }
}
