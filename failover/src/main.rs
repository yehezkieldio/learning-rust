use std::error::Error;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
struct DatabaseError {
    message: String,
}

/// Implements the `fmt::Display` trait for the `DatabaseError` struct.
///
/// This allows `DatabaseError` instances to be formatted as strings using the `{}`
/// format specifier. The formatted string will include the error message contained
/// within the `DatabaseError` instance.
///
/// # Example
///
/// ```
/// let error = DatabaseError { message: String::from("Connection failed") };
/// println!("{}", error); // Outputs: Database error: Connection failed
/// ```
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database error: {}", self.message)
    }
}

impl Error for DatabaseError {}

fn main() {
    let data: &str = "Hello, World!";

    let max_retries: u32 = 5;
    let retry_delay: Duration = Duration::from_secs(2);

    insert_with_failover(data, max_retries, retry_delay);
}

fn insert_to_database(data: &str) -> Result<(), Box<dyn Error>> {
    if rand::random::<u8>() % 2 == 0 {
        println!("Failed to insert data '{}'", data);
        Err(Box::new(DatabaseError {
            message: format!("Failed to insert data: {}", data),
        }))
    } else {
        println!("Data '{}' inserted successfully!", data);
        Ok(())
    }
}

fn insert_with_failover(data: &str, retries: u32, delay: Duration) {
    let mut attempts: u32 = 0;

    loop {
        attempts += 1;

        match insert_to_database(data) {
            Ok(_) => {
                println!("Data inserted successfully after {} attempts", attempts);
                break;
            }
            Err(e) => {
                if attempts >= retries {
                    println!("Failed to insert data after {} attempts: {}", retries, e);
                    break;
                } else {
                    println!("Failed to insert data, retrying in {:?}...", delay);
                    std::thread::sleep(delay);
                }
            }
        }
    }
}
