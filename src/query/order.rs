use std::fmt::Display;

/**
Represents a single ordering statement used in `Query` Structures

These can be combined in those queries to build more complex orderings

# Examples
Using `Order` statements is as simple as this:
```rust
use powerplatform_dataverse_service_client::query::{order::Order, Query};

let query = Query::new("contacts")
    .order(vec![Order::Ascending("lastname")]);
```
*/
#[derive(Clone, Debug)]
pub enum Order {
    /// Indicates an ascending order
    Ascending(&'static str),

    /// Indicates a descending order
    Descending(&'static str),
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Order::*;
        match self {
            Ascending(name) => f.write_fmt(format_args!("{} asc", name)),
            Descending(name) => f.write_fmt(format_args!("{} desc", name)),
        }
    }
}
