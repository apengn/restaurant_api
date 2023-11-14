use std::{error::Error as StdError, fmt};

use axum::BoxError;

#[derive(Debug)]
pub struct Error {
    inner: BoxError,
}

impl Error {
    /// Create a new `Error` from a boxable error.
    pub fn new(inner: impl Into<BoxError>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl StdError for Error {}
