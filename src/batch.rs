use std::fmt::{Display, Write};

use uuid::Uuid;

use crate::{
    client::VERSION,
    entity::WriteEntity,
    reference::Reference,
    result::{IntoDataverseResult, Result},
};

/**
Represents a batch of Microsoft Dataverse Requests

Some restrictions apply for creating batches:
- the batch size may not exceed 1000 calls
- the batch execution time may not exceed 2 minutes

the second restriction is especially tricky to handle because the execution time
depends on the complexity of the entity in dataverse.
So it is possible to create 300 records of an entity with low complexity
but only 50 records of an entity with high complexity in that timeframe.

Based on experience a batch size of 50 should be safe for all entities though 

# Examples
```rust
let testy_contact = Contact {
    contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap(),
    firstname: String::from("Testy"),
    lastname: String::from("McTestface"),
};

let marianne_contact = Contact {
    contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789abc").unwrap(),
    firstname: String::from("Marianne"),
    lastname: String::from("McTestface"),
};

// this batch creates both contacts in one call
let mut batch = Batch::new("https://instance.crm.dynamics.com/");
batch.create(&testy_contact).unwrap();
batch.create(&marianne_contact).unwrap();

client.execute(&batch).unwrap();

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
pub struct Batch {
    url: &'static str,
    batch_id: Uuid,
    dataset_id: Uuid,
    payload: String,
    next_content_id: u16,
}

impl Batch {

    /// Creates a new empty batch with its own batch id and dataset id
    pub fn new(url: &'static str) -> Self {
        Self {
            url,
            batch_id: Uuid::new_v4(),
            dataset_id: Uuid::new_v4(),
            payload: String::new(),
            next_content_id: 1,
        }
    }

    /**
    Clears the batch of its contents and generates a new batch id and
    a new dataset id

    Note that this can be used to prevent frequent allocations by reusing
    the `Batch` instance and its buffer
    */
    pub fn reset(&mut self) {
        self.batch_id = Uuid::new_v4();
        self.dataset_id = Uuid::new_v4();
        self.payload.clear();
        self.next_content_id = 1;
    }

    /// returns the current batch id (This can change after a call to `reset()` though)
    pub fn get_batch_id(&self) -> Uuid {
        self.batch_id
    }

    /// returns the current dataset id (This can change after a call to `reset()` though)
    pub fn get_dataset_id(&self) -> Uuid {
        self.dataset_id
    }

    /// returns the current count auf requests in this batch
    pub fn get_count(&self) -> u16 {
        self.next_content_id - 1
    }

    /**
    Adds a Create Request for the given entity to this batch

    Please note that this function can fail if a serde serialization error occurs

    # Examples
    ```rust
    let testy_contact = Contact {
        contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap(),
        firstname: String::from("Testy"),
        lastname: String::from("McTestface"),
    };

    let marianne_contact = Contact {
        contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789abc").unwrap(),
        firstname: String::from("Marianne"),
        lastname: String::from("McTestface"),
    };

    // this batch creates both contacts in one call
    let mut batch = Batch::new("https://instance.crm.dynamics.com/");
    batch.create(&testy_contact).unwrap();
    batch.create(&marianne_contact).unwrap();

    ```
    */
    pub fn create(&mut self, entity: &impl WriteEntity) -> Result<()> {
        let reference = entity.get_reference();
        let entity = serde_json::to_string(entity).into_dataverse_result()?;

        write!(
            self.payload,
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nPOST {}api/data/v{}/{} HTTP/1.1\nContent-Type: application/json;type=entry\n\n{}\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            entity
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }

    /**
    Adds an Update Request for the given entity to this batch

    Please note that this function can fail if a serde serialization error occurs

    # Examples
    ```rust
    let testy_contact = Contact {
        contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap(),
        firstname: String::from("Testy"),
        lastname: String::from("McTestface"),
    };

    let marianne_contact = Contact {
        contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789abc").unwrap(),
        firstname: String::from("Marianne"),
        lastname: String::from("McTestface"),
    };

    // this batch updates both contacts in one call
    let mut batch = Batch::new("https://instance.crm.dynamics.com/");
    batch.update(&testy_contact).unwrap();
    batch.update(&marianne_contact).unwrap();

    ```
    */
    pub fn update(&mut self, entity: &impl WriteEntity) -> Result<()> {
        let reference = entity.get_reference();
        let entity = serde_json::to_string(entity).into_dataverse_result()?;

        write!(
            self.payload,
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nPATCH {}api/data/v{}/{}({}) HTTP/1.1\nContent-Type: application/json;type=entry\nIf-Match: *\n\n{}\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            reference.entity_id,
            entity
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }

    /**
    Adds an Upsert Request for the given entity to this batch

    Please note that this function can fail if a serde serialization error occurs

    # Examples
    ```rust
    let testy_contact = Contact {
        contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap(),
        firstname: String::from("Testy"),
        lastname: String::from("McTestface"),
    };

    let marianne_contact = Contact {
        contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789abc").unwrap(),
        firstname: String::from("Marianne"),
        lastname: String::from("McTestface"),
    };

    // this batch creates both contacts in one call
    let mut batch = Batch::new("https://instance.crm.dynamics.com/");
    batch.upsert(&testy_contact).unwrap();
    batch.upsert(&marianne_contact).unwrap();

    ```
    */
    pub fn upsert(&mut self, entity: &impl WriteEntity) -> Result<()> {
        let reference = entity.get_reference();
        let entity = serde_json::to_string(entity).into_dataverse_result()?;

        write!(
            self.payload,
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nPATCH {}api/data/v{}/{}({}) HTTP/1.1\nContent-Type: application/json;type=entry\n\n{}\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            reference.entity_id,
            entity
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }

    /**
    Adds a Delete Request for the given entity reference to this batch

    Please note that this function can fail if a serde serialization error occurs

    # Examples
    ```rust
    let testy_reference = ReferenceStruct::new(
        "contacts",
        Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap()
    );

    let marianne_reference = ReferenceStruct::new(
        "contacts",
        Uuid::parse_str("12345678-1234-1234-1234-123456789abc").unwrap()
    );

    // this batch creates both contacts in one call
    let mut batch = Batch::new("https://instance.crm.dynamics.com/");
    batch.delete(&testy_reference).unwrap();
    batch.delete(&marianne_reference).unwrap();

    ```
    */
    pub fn delete(&mut self, entity: &impl Reference) -> Result<()> {
        let reference = entity.get_reference();

        write!(
            self.payload,
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nDELETE {}api/data/v{}/{}({}) HTTP/1.1\n\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            reference.entity_id
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }
}

impl Display for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let batch_id = self.batch_id.as_simple();
        let dataset_id = self.dataset_id.as_simple();

        f.write_fmt(
            format_args!(
                "--batch_{}\nContent-Type: multipart/mixed; boundary=changeset_{}\n\n{}--changeset_{}--\n--batch_{}--",
                batch_id,
                dataset_id,
                self.payload,
                dataset_id,
                batch_id,
            )
        )
    }
}
