/*!
crate for connecting and manipulating dataverse environments

## Creating a client and connecting to a dataverse environment

Here is an example for creating a client and authenticating via the client/secret method

```rust
use powerplatform_dataverse_service_client::client::Client;

let client_id = String::from("<clientid>");
let client_secret = String::from("<clientsecret>");

let client = Client::with_client_secret_auth(
    "https://instance.crm.dynamics.com/",
    "12345678-1234-1234-1234-123456789012",
    client_id,
    client_secret,
);
```

where the first parameter is the organization-url and the second parameter is
the tenant-id, where the client shall be authenticated against

## Reading a contact record from dataverse

To read a record from dataverse you first need to create a struct and implement ReadableEntity for it:

```rust
use serde::Deserialize;
use uuid::Uuid;
use powerplatform_dataverse_service_client::{
    entity::ReadEntity,
    select::Select
};

#[derive(Deserialize)]
struct Contact {
    contactid: Uuid,
    firstname: String,
    lastname: String,
}

impl ReadEntity for Contact {}

impl Select for Contact {
    fn get_columns() -> &'static [&'static str] {
        &["contactid", "firstname", "lastname"]
    }
}
```

The Select trait filters the retrieved columns for the ones that are relevant to the struct

then you can use the retrieve method in the client to retrieve it:

```rust
use serde::Deserialize;
use uuid::Uuid;
use powerplatform_dataverse_service_client::{
    client::Client,
    entity::ReadEntity,
    reference::ReferenceStruct,
    result::{IntoDataverseResult, Result},
    select::Select
};

async fn test() -> Result<()> {
    let client = Client::new_dummy(); // Please replace this with your preferred authentication method
    let contact: Contact = client
        .retrieve(
            &ReferenceStruct::new(
                "contacts",
                Uuid::parse_str("12345678-1234-1234-1234-123456789012").into_dataverse_result()?
            )
        )
        .await?;
    Ok(())
}

#[derive(Deserialize)]
struct Contact {
    contactid: Uuid,
    firstname: String,
    lastname: String,
}

impl ReadEntity for Contact {}

impl Select for Contact {
    fn get_columns() -> &'static [&'static str] {
        &["contactid", "firstname", "lastname"]
    }
}
```

## Writing a contact record into dataverse

To write a record into dataverse you need to create a struct and implement WritableEntity for it:

```rust
use serde::Serialize;
use uuid::Uuid;
use powerplatform_dataverse_service_client::{
    entity::WriteEntity,
    reference::{Reference, ReferenceStruct}
};

#[derive(Serialize)]
struct Contact {
    contactid: Uuid,
    firstname: String,
    lastname: String,
}

impl WriteEntity for Contact {}

impl Reference for Contact {
    fn get_reference(&self) -> ReferenceStruct {
        ReferenceStruct::new(
            "contacts",
            self.contactid,
        )
    }
}
```

where the Reference trait handles the conversion from the entity into a reference to itself in dataverse

then you are able to write it with the create method:

```rust
use uuid::Uuid;
use serde::Serialize;
use powerplatform_dataverse_service_client::client::Client;
use powerplatform_dataverse_service_client::entity::WriteEntity;
use powerplatform_dataverse_service_client::reference::{Reference, ReferenceStruct};
use powerplatform_dataverse_service_client::result::{IntoDataverseResult, Result};

async fn test() -> Result<Uuid> {
    let contact = Contact {
        contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789012").into_dataverse_result()?,
        firstname: String::from("Testy"),
        lastname: String::from("McTestface"),
    };

    let client = Client::new_dummy(); // Please replace this with your preferred authentication method
    client.create(&contact).await
}

#[derive(Serialize)]
struct Contact {
    contactid: Uuid,
    firstname: String,
    lastname: String,
}

impl WriteEntity for Contact {}

impl Reference for Contact {
    fn get_reference(&self) -> ReferenceStruct {
        ReferenceStruct::new(
            "contacts",
            self.contactid,
        )
    }
}
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
