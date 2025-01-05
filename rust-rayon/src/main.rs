use std::time::{Duration, Instant};

use rand::Rng;
use rayon::prelude::*;

fn main() {
    let size: i32 = 500_000;
    let mut rng = rand::thread_rng();
    let data: Vec<i32> = (0..size).map(|_| rng.gen_range(0..100)).collect();

    println!("Generated a vector of {} elements", size);

    // Measure single-threaded sum
    let start: Instant = Instant::now();
    let sum_single: u64 = data.par_iter().map(|&x| x as u64).sum();
    let duration_single: Duration = start.elapsed();
    println!(
        "Single-threaded sum: {}, took {}",
        sum_single,
        duration_single.as_secs_f64()
    );

    // Measure parallel sum
    let start: Instant = Instant::now();
    let sum_parallel: u64 = data.par_iter().map(|&x| x as u64).sum();
    let duration_parallel: Duration = start.elapsed();
    println!(
        "Parallel sum: {}, took {}",
        sum_parallel,
        duration_parallel.as_secs_f64()
    );

    assert_eq!(sum_single, sum_parallel);

    println!("Performance Comparison: ");
    println!(
        "Speedup: {}",
        duration_single.as_secs_f64() / duration_parallel.as_secs_f64()
    );
}
