#![feature(test)]

extern crate test;


#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::borrow::Cow;
    use std::ptr::eq;
    use std::vec;

    fn min_sum_cow(min: i32, v: &mut Cow<[i32]>) {
        let sum: i32 = v.iter().sum();
        if sum < min {
            v.to_mut().push(min - sum);
        }
    }

    fn min_sum_refcell(min: i32, v: &RefCell<Vec<i32>>) {
        let sum: i32 = v.borrow().iter().sum();
        if sum < min {
            v.borrow_mut().push(min - sum);
        }
    }

    fn min_sum_cell(min: i32, v: &Cell<Vec<i32>>) {
        let mut vec = v.take();
        let sum: i32 = vec.iter().sum();
        if sum < min {
            vec.push(min - sum);
        }
        v.set(vec);
    }

    #[test]
    fn about_cells() {
        let ref_cell = RefCell::new(vec![10, 20, 30]);
        min_sum_refcell(79, &ref_cell);

        assert!(ref_cell.borrow().eq(&vec![10, 20, 30 , 10]));

        let cell = Cell::from(vec![10, 20, 30]);
        min_sum_cell(70, &cell);

        let v = cell.into_inner();
        assert_eq!(v, vec![10, 20, 30, 10]);
    }

    #[test]
    #[should_panic]
    fn failing_cells() {
        let ref_cell = RefCell::new(vec![10, 20, 30]);
        let _v = ref_cell.borrow();
        min_sum_refcell(60, &ref_cell);

        min_sum_refcell(70, &ref_cell);
    }
}
