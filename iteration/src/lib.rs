#[cfg(test)]
mod tests {
    #[test]
    fn getting_the_iterator() {
        let v = vec![10, 10, 10];
        let mut iter = v.iter();

        // In rust, iterators over a vector of items yield references to the items.
        // This is because the iterator is borrowing the vector.
        // This means that the iterator is not the owner of the items, it is just borrowing them.
        // This is why the type of the iterator is `std::slice::Iter<'_, i32>`.
        // The `'_' is a lifetime parameter that means that the iterator borrows the vector.
        // The `i32` is the type of the items in the vector.

        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&10));

        for i in v {
            assert_eq!(i, 10);
        }
    }
}
