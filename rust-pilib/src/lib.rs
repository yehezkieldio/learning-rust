use rand::prelude::*;

pub fn monte_carlo_pi(iterations: usize) -> f32 {
    let mut inside_circle = 0;

    for _ in 0..iterations {
        // generate two random coordinates betwen 0 and 1
        let x: f32 = random::<f32>();
        let y: f32 = random::<f32>();

        // calculate the circular distance from 0,0
        if x.powi(2) + y.powi(2) <= 1_f32 {
            // if it's within a circle, increase the count inside the count
            inside_circle += 1;
        }
    }
    // return the ratio of 4 times the hits to the total iterations
    4_f32 * inside_circle as f32 / iterations as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_reasonable_pi(pi: f32) -> bool {
        // pi is between 3 and 4
        pi > 3_f32 && pi < 4.5_f32
    }

    #[test]
    fn test_monte_carlo_pi() {
        let pi = monte_carlo_pi(1);
        assert!(pi == 0_f32 || pi == 4_f32);
    }

    #[test]
    fn test_monte_carlo_pi_500() {
        let pi = monte_carlo_pi(500);
        assert!(is_reasonable_pi(pi));
    }

    #[test]
    fn test_monte_carlo_pi_1000() {
        let pi = monte_carlo_pi(1000);
        assert!(is_reasonable_pi(pi));
    }

    #[test]
    fn test_monte_carlo_pi_10000() {
        let pi = monte_carlo_pi(10000);
        assert!(is_reasonable_pi(pi));
    }
}
