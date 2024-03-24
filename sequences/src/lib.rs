#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn exploring_arrays() {
        let mut arr: [usize; 3] = [0; 3];
        assert_eq!(arr, [0, 0, 0]);

        let arr2: [usize; 5] = [1, 2, 3, 4, 5];
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        arr[0] = 1;
        assert_eq!(arr, [1, 0, 0]);
        assert_eq!(arr[0], 1);
        assert_eq!(mem::size_of_val(&arr), mem::size_of::<usize>() * 3);
    }

    struct Point(f32, f32);

    #[test]
    fn exploring_tuples() {
        let mut my_tuple: (i32, usize, f32) = (10, 0, -3.42);

        assert_eq!(my_tuple.0, 10);
        assert_eq!(my_tuple.1, 0);
        assert_eq!(my_tuple.2, -3.42);

        my_tuple.0 = 100;
        assert_eq!(my_tuple.0, 100);

        let (_val1, _val2, _val3) = my_tuple;

        let point = Point(1.2, 2.1);
        assert_eq!(point.0, 1.2);
        assert_eq!(point.1, 2.1);
    }

    #[test]
    fn exploring_vec() {
        assert_eq!(vec![0; 3], [0, 0, 0]);
        let mut v: Vec<i32> = vec![];

        assert_eq!(mem::size_of::<Vec<i32>>(), mem::size_of::<usize>() * 3);
        assert_eq!(mem::size_of_val(&*v), 0);

        v.push(10);

        assert_eq!(mem::size_of::<Vec<i32>>(), mem::size_of::<i32>() * 6);

        assert_eq!(v[0], 10);
        v.insert(0, 11);
        v.push(12);
        assert_eq!(v, [11, 10, 12]);
        assert!(!v.is_empty());

        assert_eq!(v.swap_remove(0), 11);
        assert_eq!(v, [12, 10]);

        assert_eq!(v.pop(), Some(10));
        assert_eq!(v, [12]);

        assert_eq!(v.remove(0), 12);
        v.shrink_to_fit();
        assert_eq!(mem::size_of_val(&*v), 0);
    }
}
