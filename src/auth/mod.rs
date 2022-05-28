/*!
Module for handling authentication against Microsoft Dataverse instances

The main trait here is the `Authenticate` trait which encapsulates a method
of acquiring a token for Bearer authentication:

```rust
#[async_trait]
pub trait Authenticate {
    async fn get_valid_token(&self) -> Result<Arc<String>>;
}
```
*/

use std::sync::Arc;

use async_trait::async_trait;

use crate::result::Result;

pub mod client_secret;

/**
trait for methods that result in the acquisition of tokens usable
in Bearer token authentication for Microsoft Dataverse calls

see `get_valid_token(...)` for more details
*/
#[async_trait]
pub trait Authenticate {

    /**
    Authenticates the current instance and returns the Bearer token to use
    in subsequent Microsoft Dataverse calls

    Aquired tokens should be cached and reused as long as they are valid
    and then refreshed when necessary

    Implementations should propagate hard authentication errors but may
    handle soft errors with their own strategies like retries
    */
    async fn get_valid_token(&self) -> Result<Arc<String>>;
}
