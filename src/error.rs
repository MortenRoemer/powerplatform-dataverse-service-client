use std::{error::Error, fmt::Display};

/**
The Error that is returned if any of the operations in this crate
fails.
*/
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataverseError {
    pub message: String,
}

impl DataverseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for DataverseError {}

impl Display for DataverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
