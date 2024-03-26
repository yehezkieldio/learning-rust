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

    fn count_files(path: &String) -> usize {
        path.len()
    }

    #[test]
    fn data_transformations() {
        let v = vec![10, 10, 10];
        let hexed = v.iter().map(|i| format!("{:x}", i));
        assert_eq!(
            hexed.collect::<Vec<String>>(),
            vec!["a".to_string(), "a".to_string(), "a".to_string()]
        );

        // fold is a method that takes an initial value and a closure.
        // The closure takes two arguments: an accumulator and an element.
        // In this case, p is the accumulator and c is the element.
        // An accumulator is a value that is updated with the result of the closure.
        // The closure returns the new value of the accumulator.
        // The fold method returns the final value of the accumulator.
        // In this case, the initial value is 0 and the closure adds the element to the accumulator.
        // The final value is 30.
        assert_eq!(v.iter().fold(0, |p, c| p + c), 30);

        let dirs = vec![
            "/home/alice".to_string(),
            "/home/bob".to_string(),
            "/home/carl".to_string(),
            "/home/debra".to_string(),
        ];

        let file_counter = dirs.iter().map(count_files);

        // The zip method takes two iterators and returns an iterator that yields pairs of elements.
        // In this case, the first iterator is the dirs iterator and the second iterator is the file_counter iterator.

        // or, the zip function in rust takes two iterators and returns an iterator that yields pairs of elements or tuples.
        // where each pair or tuple contains one element from each of the input iterators.
        let dir_file_counts: Vec<(&String, usize)> = dirs.iter().zip(file_counter).collect();

        // the vec! contains tuples of the directory and the number of files in the directory.
        // the number of files is the length of the directory string.
        // and the directory is the directory string that is referenced by the iterator.
        assert_eq!(
            dir_file_counts,
            vec![
                (&"/home/alice".to_string(), 11),
                (&"/home/bob".to_string(), 9),
                (&"/home/carl".to_string(), 10),
                (&"/home/debra".to_string(), 11),
            ]
        )
    }

    #[test]
    fn data_filtering() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

        // The filter method takes a closure that returns a boolean.
        // The filter method returns an iterator that yields only the elements for which the closure returns true.
        // In this case, the closure checks if the element is even.
        // The filter method returns an iterator that yields only the even numbers.
        // The all method checks if all the elements in the iterator are even.
        // The all method returns true if all the elements are even.
        assert!(data.iter().filter(|&n| n % 2 == 0).all(|&n| n % 2 == 0));

        // Double ampersand is used to dereference the reference or reference to a reference.
        // In other words, it is used to get the value that the reference points to.
        // And the single ampersand is used to get the reference to the value.
        assert_eq!(data.iter().find(|&&n| n == 5), Some(&5));
        assert_eq!(data.iter().find(|&&n| n == 0), None);
        assert_eq!(data.iter().position(|&n| n == 5), Some(4));

        // skip method skips the first n elements of the iterator.
        assert_eq!(data.iter().skip(1).next(), Some(&2));

        // take method takes the first n elements of the iterator.
        let mut data_iter = data.iter().take(2);
        assert_eq!(data_iter.next(), Some(&1));
        assert_eq!(data_iter.next(), Some(&2));
        assert_eq!(data_iter.next(), None);

        // The partition method takes a closure that returns a boolean.
        // The partition method returns a tuple of two iterators.
        // This partition method returns two iterators: one for the elements that are even and one for the elements that are odd.
        let (validation, train): (Vec<i32>, Vec<i32>) = data
            .iter()
            .partition(|&_| (rand::random::<f32>() % 1.0) > 0.8);

        assert!(train.len() > validation.len());
    }
}
