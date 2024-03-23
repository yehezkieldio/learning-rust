// Rust allows another macro type: derive. It allows to "auto-implement"
// supported traits. Clone, Debug, Copy are typically handy to derive.
#[derive(Clone, Debug, Copy)]
struct MyCustomStruct {
    a: i32,
    b: u32,
    pub c: f32,
}

// A typical Rust struct has an impl block for behavior
impl MyCustomStruct {
    // The new function is static function, and by convention a
    // constructor
    pub fn new(a: i32, b: u32, c: f32) -> MyCustomStruct {
        MyCustomStruct { a: a, b: b, c: c }
    }

    // Instance functions feature a "self" reference as the first parameter
    // This self reference can be mutable or owned, just like other variables
    pub fn sum(&self) -> f32 {
        self.a as f32 + self.b as f32 + self.c
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
    fn basic_math_stuffs() {
        assert_eq!(2 + 2, 4);
        assert_eq!(3.14 + 22.86, 26_f32);
        assert_eq!(2_i32.pow(2), 4);
        assert_eq!(4_f32.sqrt(), 2_f32);

        let a: u64 = 32;
        let b: u64 = 64;

        // Risky, this could overflow
        assert_eq!(a - b, 32);
        assert_eq!(a.overflowing_add(b), (18446744073709551584, true));
        let mut c = 100;
        c += 1;
        assert_eq!(c, 101);
    }

    #[test]
    #[should_panic]
    fn attempt_overflow() {
        let a: u64 = 32;
        let b: u64 = 64;
        let _ = a - b;
    }
}
