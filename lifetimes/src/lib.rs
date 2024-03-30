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

pub struct StatisticToolkit<'a> {
    base: &'a [f64],
}

impl<'a> StatisticToolkit<'a> {
    pub fn new(base: &'a [f64]) -> Option<StatisticToolkit> {
        if base.len() < 3 {
            None
        } else {
            Some(StatisticToolkit { base })
        }
    }

    // This function should return the mean of the numbers in the slice.
    pub fn var(&self) -> f64 {
        let mean = self.mean();
        let ssq: f64 = self.base.iter().map(|i| (i - mean).powi(2)).sum();
        return ssq / self.base.len() as f64;
    }

    // This function should return the standard deviation of the numbers in the slice.
    pub fn std(&self) -> f64 {
        self.var().sqrt()
    }

    // This function should return the mean of the numbers in the slice.
    pub fn mean(&self) -> f64 {
        self.base.iter().sum::<f64>() / self.base.len() as f64
    }

    // This function should return the median of the numbers in the slice.
    pub fn median(&self) -> f64 {
        let mut clone = self.base.to_vec();

        clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let m = clone.len() / 2;
        if clone.len() % 2 == 0 {
            (clone[m - 1] + clone[m]) / 2.0
        } else {
            clone[m]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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

    #[test]
    fn mean_tests() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
        assert_eq!(mean(&vec![]), None);
        assert_eq!(mean(&vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]), Some(0.0));
    }

    #[test]
    fn test_statistic_toolkit() {
        assert!(StatisticToolkit::new(&vec![]).is_none());
        assert!(StatisticToolkit::new(&vec![1.0, 2.0]).is_none());

        assert!(StatisticToolkit::new(&vec![1.0, 2.0, 3.0]).is_some());
        let binding = vec![1.0, 2.0, 3.0];
        let toolkit = StatisticToolkit::new(&binding).unwrap();
        assert_eq!(toolkit.mean(), 2.0);
        assert_eq!(toolkit.var(), 0.6666666666666666);
        assert_eq!(toolkit.std(), 0.816496580927726);
        assert_eq!(toolkit.median(), 2.0);
    }
}
