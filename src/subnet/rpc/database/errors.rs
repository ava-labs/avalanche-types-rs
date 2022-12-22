//! Custom database errors and helpers.
use std::io::{Error, ErrorKind};

/// Accepts an error and returns a corruption error if the original error is not "database closed"
/// or "not found".
pub async fn is_corruptible(error: Error) -> (bool, Error) {
    match error {
        e if database_is_closed(&e) => (false, e),
        e if is_not_found(&e) => (false, e),
        _ => (
            true,
            Error::new(
                ErrorKind::Other,
                format!("closed to avoid possible corruption, init error: {}", error),
            ),
        ),
    }
}

/// Returns false if the io::Error is ErrorKind::NotFound and contains a string "not found".
pub fn is_not_found(error: &Error) -> bool {
    error.kind() == ErrorKind::NotFound && error.to_string().contains("not found")
}

/// Returns a standardized io::Error to indicate a key is not found.
pub fn not_found() -> Error {
    Error::new(ErrorKind::NotFound, "not found")
}

/// Returns false if the io::Error message contains the string "database closed".
pub fn database_is_closed(error: &Error) -> bool {
    if error.to_string().contains("database closed") {
        return true;
    }
    false
}

/// Returns a standardized io::Error to indicate the database is closed.
pub fn database_closed() -> Error {
    Error::new(ErrorKind::Other, "database closed")
}

/// Returns an io::Error with ErrorKind::Other from a string.
pub fn from_string(message: String) -> Error {
    Error::new(ErrorKind::Other, message)
}
