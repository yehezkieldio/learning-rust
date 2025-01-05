use gcd_core::gcd::compute_gcd;

struct CLIArgs {
    function: String,
    m: u64,
    n: u64,
}

fn main() {
    let function = std::env::args().nth(1).expect("function name is required");
    let m = std::env::args().nth(2).expect("m is required");
    let n = std::env::args().nth(3).expect("n is required");

    let args = CLIArgs {
        function,
        m: m.parse().expect("m must be a number"),
        n: n.parse().expect("n must be a number"),
    };

    let gcd = match args.function.as_str() {
        "gcd" => compute_gcd(args.m, args.n),
        _ => panic!("unknown function"),
    };

    println!("gcd({},{}) = {}", args.m, args.n, gcd);
}
