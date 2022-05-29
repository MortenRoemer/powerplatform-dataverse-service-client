use std::sync::Arc;

use async_trait::async_trait;

use crate::{result::Result, error::DataverseError};
use super::Authenticate;

/**
Implements the `Authenticate` trait by failing it consistently

This only exists for unit-testing and doc-testing purposes where you would want to prevent
a huge amount of erronous calls to authentication servers
*/
pub struct NoAuth {}

#[async_trait]
impl Authenticate for NoAuth {
    async fn get_valid_token(&self) -> Result<Arc<String>> {
        Err(DataverseError::new(String::from("No authentication method selected. This is here for testing purposes. please select another auth method")))
    }
}