use std::io::{self};

/// Represents different kinds of application errors.
pub enum ApplicationError {
    /// An error with a code.
    Code {
        full: usize, // The full code value.
        short: u16,  // The short code value.
    },
    /// An unknown error.
    Unknown,
    /// An error with a message.
    Message(String),
    /// An error related to I/O operations.
    IOWrapper(io::Error),
}

impl ApplicationError {
    /// Prints the kind of the error to the given output stream.
    ///
    /// # Arguments
    ///
    /// * `to` - A mutable reference to an object that implements the `io::Write` trait.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the kind is successfully written to the output stream, or an `io::Error` if an error occurs.
    pub fn print_kind(&self, mut to: &mut impl io::Write) -> io::Result<()> {
        let kind = match self {
            ApplicationError::Code { full: _, short: __ } => "Code",
            ApplicationError::Unknown => "Unknown",
            ApplicationError::Message(_) => "Message",
            ApplicationError::IOWrapper(_) => "IOWrapper",
        };

        write!(&mut to, "{}", kind)?;
        Ok(())
    }
}

/// Performs some work based on the given choice.
///
/// # Arguments
///
/// * `choice` - An integer representing the choice.
///
/// # Returns
///
/// Returns `Ok(())` if the work is successfully performed, or an `ApplicationError` if an error occurs.
pub fn do_work(choice: i32) -> Result<(), ApplicationError> {
    if choice < 100 {
        Err(ApplicationError::IOWrapper(io::Error::from(
            io::ErrorKind::Other,
        )))
    } else if choice == 42 {
        Err(ApplicationError::Code {
            full: choice as usize,
            short: (choice % u16::max_value() as i32) as u16,
        })
    } else if choice > 42 {
        Err(ApplicationError::Message(format!(
            "{} lead to a terrible error",
            choice
        )))
    } else {
        Err(ApplicationError::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::{do_work, ApplicationError};
    use std::io;

    #[test]
    fn test_do_work() {
        let choice = 10;
        if let Err(error) = do_work(choice) {
            match error {
                ApplicationError::Code {
                    full: code,
                    short: _,
                } => assert_eq!(choice as usize, code),
                ApplicationError::Unknown | ApplicationError::IOWrapper(_) => assert!(choice < 42),
                ApplicationError::Message(msg) => {
                    assert_eq!(format!("{} lead to a terrible error", choice), msg)
                }
            }
        }
    }

    #[test]
    fn test_application_error_kind() {
        let mut target = vec![];

        let _ = ApplicationError::Code {
            full: 100,
            short: 100,
        }
        .print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Code");

        let mut target = vec![];
        let _ = ApplicationError::Message("0".to_string()).print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Message");

        let mut target = vec![];
        let _ = ApplicationError::Unknown.print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Unknown");

        let mut target = vec![];
        let _ = ApplicationError::IOWrapper(io::Error::from(io::ErrorKind::Other))
            .print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "IOWrapper");
    }
}
