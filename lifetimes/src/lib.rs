// Lifetimes are a way to tell the compiler that a reference is valid for a certain amount of time.
// This is important because the compiler needs to know when it is safe to deallocate memory.

// This function should return the mean of the numbers in the slice.
// Mean is the sum of all the numbers divided by the number of numbers.
// If the slice is empty, return None.
// It takes a slice of f64 numbers and returns an Option<f32>.
// The lifetime of the slice is 'a. This means that the slice is valid for the lifetime 'a.
pub fn mean<'a>(numbers: &'a [f64]) -> Option<f32> {
    if numbers.len() > 0 {
        let sum = numbers.iter().sum::<f64>();
        Some((sum / numbers.len() as f64) as f32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&numbers), Some(3.0));
    }

    #[test]
    fn test_mean_empty() {
        let numbers = vec![];
        assert_eq!(mean(&numbers), None);
    }
}
