pub mod rounding;

use rust_pilib::monte_carlo_pi;

fn main() {
    let pi = monte_carlo_pi(1000000);
    let places = 2;

    println!(
        "Pi is ~ {} and rounded to {} places {}",
        pi,
        places,
        rounding::round(pi, places)
    );
}
