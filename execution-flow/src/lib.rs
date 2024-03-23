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
    fn conditionals() {
        let i = 30;
        if i < 20 {
            assert!(i < 20);
        } else if i > 2 {
            assert!(i > 2);
        } else {
            assert_eq!(i, 2);
        }
    }

    #[test]
    fn more_conditionals() {
        let my_option = Some(10);

        if let Some(unpacked) = my_option {
            assert_eq!(unpacked, 10);
        }

        let mut other_option = Some(2);
        while let Some(unpacked) = other_option {
            other_option = if unpacked > 0 {
                Some(unpacked - 1)
            } else {
                None
            }
        }
        assert_eq!(other_option, None);
    }

    #[test]
    #[allow(unused_assignments)]
    fn loops() {
        let mut i = 42;
        let mut broke = false;

        loop {
            i -= 1;
            if i < 2 {
                broke = true;
                break;
            } else if i > 2 {
                continue;
            }
        }
        assert!(broke);

        let mut iterations: u32 = 0;

        let total_squared = loop {
            iterations += 1;

            if iterations >= 10 {
                break iterations.pow(2);
            }
        };
        assert_eq!(total_squared, 100);

        for i in 0..10 {
            assert!(i >= 0 && i < 10);
        }

        for v in vec![1, 1, 1, 1].iter() {
            assert_eq!(v, &1);
        }
    }
}
