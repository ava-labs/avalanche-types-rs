use std::{cell::RefCell, fmt, rc::Rc};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Backing errors for all consensus operations.
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("failed API (message: {message:?}, is_retryable: {is_retryable:?})")]
    API { message: String, is_retryable: bool },
    #[error("failed for other reasons (message: {message:?}, is_retryable: {is_retryable:?})")]
    Other { message: String, is_retryable: bool },
}

impl Error {
    /// Returns the error message in "String".
    #[inline]
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Error::API { message, .. } | Error::Other { message, .. } => message.clone(),
        }
    }

    /// Returns if the error is retryable.
    #[inline]
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::API { is_retryable, .. } | Error::Other { is_retryable, .. } => *is_retryable,
        }
    }

    /// Returns true if the error message contains the substring.
    #[inline]
    #[must_use]
    pub fn contains(&self, s: &str) -> bool {
        self.message().contains(s)
    }
}

#[derive(Debug)]
pub struct Errors {
    d: Rc<RefCell<Vec<Error>>>,
}

impl Errors {
    pub fn new() -> Self {
        Self {
            d: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn errored(&self) -> bool {
        !self.d.borrow().is_empty()
    }

    pub fn add(&self, e: Error) {
        self.d.borrow_mut().push(e);
    }
}

impl Default for Errors {
    fn default() -> Self {
        Self::new()
    }
}

/// ref. <https://doc.rust-lang.org/std/string/trait.ToString.html>
/// ref. <https://doc.rust-lang.org/std/fmt/trait.Display.html>
/// Use "Self.to_string()" to directly invoke this
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut errs: Vec<String> = Vec::new();
        for e in self.d.borrow().iter() {
            errs.push(e.message());
        }
        write!(f, "{}", errs.join(", "))
    }
}
