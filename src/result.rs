use std::fmt::Display;

use crate::error::DataverseError;

pub struct ResultCollection<E> {
    pub entities: Vec<Result<E>>,
}

pub type Result<T> = std::result::Result<T, DataverseError>;

pub trait IntoDataverseResult<T> {
    fn into_dataverse_result(self) -> Result<T>;
}

impl<T, E: Display> IntoDataverseResult<T> for core::result::Result<T, E> {
    fn into_dataverse_result(self) -> Result<T> {
        self.map_err(|error| DataverseError::new(error.to_string()))
    }
}
