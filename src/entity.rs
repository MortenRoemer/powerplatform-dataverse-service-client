use serde::{de::DeserializeOwned, Serialize};

use crate::{reference::Reference, select::Select};

/**
Supertrait for entities that can be retrieved from a Microsoft
Dataverse environment

This should be implemented by data structures you want to use with
the following functions in `Client`:
- `retrieve(...)`
- `retrieve_multiple(...)`

# Examples
```rust
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
*/
pub trait ReadEntity: DeserializeOwned + Select {}

/**
Supertrait for entities that can be written into a Microsoft
Dataverse environment

This should be implemented by data structures you want to use with
the following functions in `Client`:
- `create(...)`
- `update(...)`
- `upsert(...)`

# Examples
```rust
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
pub trait WriteEntity: Serialize + Reference {}
