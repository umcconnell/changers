use std::{fmt, io};

/// Custom error type for CLI errors
///
/// Used mainly when users pass invalid arguments.
///
/// # Examples
///
/// When passing in invalid monetary amounts, such as `error` or `no_amount`,
/// this error will be thrown.
#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Args(&'static str),
    File(&'static str),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Args(ref msg) => write!(f, "{}", msg),
            CliError::File(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}
