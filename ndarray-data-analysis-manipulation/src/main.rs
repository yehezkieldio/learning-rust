use ndarray::{Array, ShapeBuilder};

fn initial_placeholders() {
    // create a 1x4 array of zeros
    // 1x4 array is a 1D array with 4 elements
    // 1D array is basically a vector in Rust
    let zeros = Array::<f64, _>::zeros((1, 4).f());
    println!("{:?}", zeros);

    // create a 1x4 array of ones
    let ones = Array::<f64, _>::ones((1, 4));
    println!("{:?}", ones);

    // create a 1x4 array with values in the range 0..4
    let range = Array::<f64, _>::range(0., 4., 1.);
    println!("{:?}", range);

    // create a 1x4 array with values evenly spaced between 0 and 4
    // evenly spaced values means the difference between any two consecutive values is the same
    // for example, 0, 1, 2, 3, 4 is evenly spaced or 0, 2, 4, 6, 8 is evenly spaced
    let linspace = Array::<f64, _>::linspace(0., 4., 4);
    println!("{:?}", linspace);

    let mut ones = Array::<f64, _>::ones((1, 4));
    ones.fill(0.);
    println!("{:?}", ones);

    // eye is a 4x4 identity matrix with 1s on the diagonal and 0s elsewhere
    let eye = Array::<f64, _>::eye(4);
    println!("{:?}", eye);
}

fn main() {
    initial_placeholders();
}
