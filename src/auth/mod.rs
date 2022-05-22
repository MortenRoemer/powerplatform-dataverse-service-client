use std::sync::Arc;

use async_trait::async_trait;

use crate::result::Result;

pub mod client_secret;

#[async_trait]
pub trait Authenticate {
    async fn get_valid_token(&self) -> Result<Arc<String>>;
}
