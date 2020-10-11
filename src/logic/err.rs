use std::fmt;

/// Custom error type for logic errors
///
/// Used mainly when change cannot be made due to mathematical impossibility
///
/// # Examples
///
/// When trying to make change for `2.5` with the available coins `1, 2, 5, 10`
/// this error will be thrown.
#[derive(Debug)]
pub enum LogicError {
    Unreachable(&'static str),
}

impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogicError::Unreachable(ref msg) => write!(f, "{}", msg),
        }
    }
}
