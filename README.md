# powerplatform-dataverse-service-client
Unofficial Rust client for connecting to a Microsoft Dataverse environment

Please report any issues here: https://github.com/MortenRoemer/powerplatform-dataverse-service-client/issues

planned (⏳) and completed (✅) features
- ⏳ Generic OAuth2 authentication
- ✅ Client/Secret authentication
- ✅ Basic CRUD operations
- ✅ Batch operations
- ⏳ Custom Action calls
- ⏳ Advanced ODATA query options
- ⏳ Navigation property handling

## Creating a client and connecting to a dataverse environment

Here is an example for creating a client and authenticating via the client/secret method

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

where the first parameter is the organization-url and the second parameter is
the tenant-id, where the client shall be authenticated against

## Reading a contact record from dataverse

To read a record from dataverse you first need to create a struct and implement ReadableEntity for it:

```rust
#[derive(Deserialize)]
struct Contact {
    contactid: Uuid,
    firstname: String,
    lastname: String,
}

impl ReadableEntity for Contact {}

impl Select for Contact {
    fn get_columns() -> &'static [&'static str] {
        &["contactid", "firstname", "lastname"]
    }
}
```

The Select trait filters the retrieved columns for the ones that are relevant to the struct

then you can use the retrieve method in the client to retrieve it:

```rust
let contact: Contact = client
    .retrieve(
        &ReferenceStruct::new(
            "contacts", 
            Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap()
        )
    )
    .await
    .unwrap();
```

## Writing a contact record into dataverse

To write a record into dataverse you need to create a struct and implement WritableEntity for it:

```rust
#[derive(Serialize)]
struct Contact {
    contactid: Uuid,
    firstname: String,
    lastname: String,
}

impl WritableEntity for Contact {}

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
let contact = Contact {
    contactid: Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap(),
    firstname: String::from("Testy"),
    lastname: String::from("McTestface"),
};

client.create(&contact).await.unwrap();
```