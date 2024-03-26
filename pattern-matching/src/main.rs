enum RealCat {
    Alive {
        hungry: bool,
    },
    #[allow(dead_code)]
    Dead,
}

fn main() {
    let real_cat = RealCat::Alive { hungry: true };
    match real_cat {
        RealCat::Alive { hungry } => {
            if hungry {
                println!("The cat is alive and hungry");
            } else {
                println!("The cat is alive and not hungry");
            }
        }
        RealCat::Dead => {
            println!("The cat is dead");
        }
    }

    let number = 15;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),

        // Regarding range expressions:
        // 13..19 is a range of numbers from 13 up to but not including 19
        // 13..=19 is a range of numbers from 13 up to and including 19
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}
