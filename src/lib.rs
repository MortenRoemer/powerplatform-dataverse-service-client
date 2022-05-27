/*!
crate for connecting and manipulating dataverse environments

The most important is the client module. you can easily create a client with this
code:
```rust
let client_id = String::from("<clientid>");
let client_secret = String::from("<clientsecret>");

let client = Client::with_client_secret_auth(
    "https://instance.crm.dynamics.com/",
    "12345678-1234-1234-1234-123456789012",
    client_id,
    client_secret,
);
```
*/

pub mod auth;
pub mod batch;
pub mod client;
pub mod entity;
pub mod error;
pub mod query;
pub mod reference;
pub mod result;
pub mod select;