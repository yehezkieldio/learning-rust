enum RealCat {
    Alive {
        hungry: bool,
    },
    #[allow(dead_code)]
    Dead,
}

fn literal_match(choice: usize) -> String {
    match choice {
        0 | 1 => "zero or two".to_string(),
        // 2..9 range pattern seems to be deprecated
        2..=9 => "two to nine".to_string(),
        10 => "ten".to_string(),
        _ => "anything else".to_string(),
    }
}

fn tuple_match(choices: (i32, i32, i32, i32)) -> String {
    match choices {
        (_, second, _, fourth) => format!(
            "Numbers at positions 1 and 3 are {} and {} respectively",
            second, fourth
        ),
    }
}

enum Background {
    Color(u8, u8, u8),
    Image(&'static str),
}

enum UserType {
    Casual,
    Power,
}

struct MyApp {
    theme: Background,
    user_type: UserType,
    sercret_user_id: usize,
}

fn destructuring_match(app: MyApp) -> String {
    match app {
        MyApp {
            user_type: UserType::Power,
            sercret_user_id: uid,
            theme: Background::Color(b1, b2, b3),
        } => format!(
            "A power user with id >{}< and color background (#{:02x}{:02x}{:02x}))",
            uid, b1, b2, b3
        ),
        MyApp {
            user_type: UserType::Power,
            sercret_user_id: uid,
            theme: Background::Image(path),
        } => format!(
            "A power user with id >{}< and image background (path: {})",
            uid, path
        ),
        MyApp {
            user_type: UserType::Casual,
            sercret_user_id: uid,
            theme: Background::Color(b1, b2, b3),
        } => format!(
            "A casual user with id >{}< and color background (#{:02x}{:02x}{:02x})",
            uid, b1, b2, b3
        ),
        MyApp {
            user_type: UserType::Casual,
            sercret_user_id: uid,
            theme: Background::Image(path),
        } => format!(
            "A casual user with id >{}< and image background (path: {})",
            uid, path
        ),
    }
}

fn guarded_match(app: MyApp) -> String {
    match app {
        MyApp {
            user_type: UserType::Power,
            sercret_user_id: uid,
            theme: Background::Color(_b1, _b2, _b3),
        } if uid <= 100 => "You are an early bird!".to_string(),
        MyApp {
            user_type: UserType::Power,
            sercret_user_id: uid,
            theme: Background::Color(_b1, _b2, _b3),
        } if uid > 100 => "You are a late comer!".to_string(),
        _ => "Thank you for joining!".to_string(),
    }
}

fn reference_match(m: &Option<&str>) -> String {
    match m {
        Some(ref s) => s.to_string(),
        _ => "Nothing".to_string(),
    }
}

fn literal_str_match(choice: &str) -> String {
    match choice {
        "hello" => "world".to_string(),
        _ => "anything else".to_string(),
    }
}

pub fn main() {
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

    println!();

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

    let opt = Some(42);
    match opt {
        Some(nr) => println!("Received {}", nr),
        _ => println!("Found None"),
    }

    println!();
    println!("Literal match for 0: {}", literal_match(0));
    println!("Literal match for 10: {}", literal_match(10));
    println!("Literal match for 100: {}", literal_match(100));

    println!();
    println!("Literal match for 0: {}", tuple_match((0, 10, 0, 100)));
    println!();

    let mystr = Some("Hello");
    println!("Matching on a reference: {}", reference_match(&mystr));
    println!("It's still owned: {:?}", mystr);
    println!();

    let power = MyApp {
        sercret_user_id: 99,
        theme: Background::Color(255, 255, 0),
        user_type: UserType::Power,
    };
    println!("Destructuring a power user {}", destructuring_match(power));

    let casual = MyApp {
        sercret_user_id: 10,
        theme: Background::Image("path/to/image.png"),
        user_type: UserType::Casual,
    };
    println!(
        "Destructuring a casual user {}",
        destructuring_match(casual)
    );

    let power2 = MyApp {
        sercret_user_id: 101,
        theme: Background::Color(255, 255, 0),
        user_type: UserType::Power,
    };
    println!(
        "Desctructuring a power user {}",
        destructuring_match(power2)
    );

    println!();
    let early = MyApp {
        sercret_user_id: 99,
        theme: Background::Color(255, 255, 0),
        user_type: UserType::Power,
    };
    println!("Guarded match for an early bird: {}", guarded_match(early));

    let late = MyApp {
        sercret_user_id: 101,
        theme: Background::Color(255, 255, 0),
        user_type: UserType::Power,
    };
    println!("Guarded match for a late comer: {}", guarded_match(late));
    println!();

    println!(
        "Literal string match for 'hello': {}",
        literal_str_match("hello")
    );
}
