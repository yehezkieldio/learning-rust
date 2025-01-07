use ndarray::{Array, Array3, ShapeBuilder, array, rcarr1};

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

fn multidimensional_arrays() {
    let array_1d = Array::from_vec(vec![1., 2., 3., 4.]);
    println!("{:?}", array_1d);

    let array_d11 = Array::from_shape_vec((1, 4), vec![1, 2, 3, 4]);
    println!("{:?}", array_d11.unwrap());

    let array_2d = array![
        [-1.01, 0.86, -4.60, 3.31, -4.81],
        [3.98, 0.53, -7.04, 5.29, 3.55],
        [3.30, 8.26, -3.89, 8.20, -1.51],
        [4.43, 4.96, -7.66, -7.33, 6.18],
        [7.31, -6.43, -6.16, 2.47, 5.58],
    ];
    println!("{:?}", array_2d);

    let array_d2 = Array::from_shape_vec((2, 2), vec![1., 2., 3., 4.]);
    // 1. or 2. with a trailing dot is a float in Rust
    println!("{:?}", array_d2.unwrap());

    let data = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
    let array_2d1 = Array::from_shape_vec((3, 3), data);
    println!("{:?}", array_2d1.unwrap());

    let data = vec![1., 2., 3., 4.];
    let array_3d = Array3::from_shape_vec((2, 2, 1), data);
    println!("{:?}", array_3d.unwrap());
}

fn reshaping_arrays() {
    // another way to create a 1D array
    let array_d1 = rcarr1(&[1., 2., 3., 4.]);
    let array_d2 = array_d1.to_shape((2, 2));
    println!("{:?}", array_d2);
}

fn main() {
    initial_placeholders();
    println!();
    multidimensional_arrays();
    println!();
    reshaping_arrays();
}
