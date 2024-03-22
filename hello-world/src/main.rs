use std::io::{self, Write};

fn main() {
    println!("Let's print some lines!");
    println!();
    println!("Hello, World!");
    println!("{}, {}!", "Hello", "World");
    print!("Hello, ");
    println!("World!");

    println!("Arguments can be referred to by their position: {0}, {1}! and {1}, {0}! are built from the same arguments", "Hello", "world");
    println!(
        "Futhermore the arguments can be named: \"{greeting}, {object}!\"",
        greeting = "Hello",
        object = "World"
    );
    println!(
        "Number formatting: Pi is {0:.3} or {0:.0} for short",
        std::f64::consts::PI
    );
    println!(
        "...and there is more: {0:>0width$}={0:>width$}={0:#x}",
        1532,
        width = 5
    );

    let _ = write!(
        &mut io::stdout(),
        "Underneath, it's all writing to a stream..."
    );
    println!();

    println!("Write something!");
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        println!("You wrote: {}", input);
    } else {
        println!("Failed to read input");
    }
}
