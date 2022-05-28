/*!
Module for defining Microsoft Dataverse queries for use in `Client::retrieve_multiple(...)`

These queries are modeled after the ODATAv4 specifications used by Microsoft Dataverse.
This section is prone to change as these specifications allow for a wide array of
possible querying options.

# Examples
```rust
let query = Query::new("contacts")
    .limit(3)
    .filter(Filter::Equal("firstname", AttributeValue::String(String::from("Testy"))))
    .order(vec![Order::Ascending("lastname")]);

let contacts = client.retrieve_multiple(&query).unwrap();

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

use std::fmt::Display;

use self::{filter::Filter, order::Order};

pub mod attribute;
pub mod filter;
pub mod order;

/**
Represents a Microsoft Dataverse query

These queries are modeled after the ODATAv4 specifications used by Microsoft Dataverse.
This section is prone to change as these specifications allow for a wide array of
possible querying options.

# Examples
```rust
let query = Query::new("contacts")
    .limit(3)
    .filter(Filter::Equal("firstname", AttributeValue::String(String::from("Testy"))))
    .order(vec![Order::Ascending("lastname")]);

let contacts = client.retrieve_multiple(&query).unwrap();

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
pub struct Query {
    pub logical_name: &'static str,
    pub limit: Option<u32>,
    pub filter: Option<Filter>,
    pub order: Option<Vec<Order>>,
}

impl Query {

    /**
    Creates a new empty query for the given table

    Please note that though it is possible to execute this empty query directly,
    this will attempt to retrieve every single entity record from the table.

    Use the `limit(...)` and `filter(...)` functions to add a limiting factor to your query
    of you don't want this
    */
    pub fn new(logical_name: &'static str) -> Self {
        Self {
            logical_name,
            limit: None,
            filter: None,
            order: None,
        }
    }

    /// limits the query result to at most `n` entities
    pub fn limit(mut self, count: u32) -> Self {
        self.limit = Some(count);
        self
    }

    /// filters the query result to records that match the predicate defined
    /// in the given filter
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// orders the query result by the given attributes and directions
    pub fn order(mut self, order: Vec<Order>) -> Self {
        self.order = Some(order);
        self
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first_item = true;
        f.write_str(self.logical_name)?;

        if let Some(limit) = self.limit {
            if first_item {
                f.write_str("?")?;
                first_item = false;
            } else {
                f.write_str("&")?;
            }

            f.write_fmt(format_args!("$top={}", limit))?;
        }

        if let Some(filter) = &self.filter {
            if first_item {
                f.write_str("?")?;
                first_item = false;
            } else {
                f.write_str("&")?;
            }

            f.write_fmt(format_args!("$filter={}", filter))?;
        }

        if let Some(order) = &self.order {
            if first_item {
                f.write_str("?")?;
            } else {
                f.write_str("&")?;
            }

            f.write_str("$orderby=")?;

            let mut first_column = true;

            for column in order.iter() {
                if first_column {
                    first_column = false;
                } else {
                    f.write_str(",")?;
                }

                f.write_fmt(format_args!("{}", column))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::query::{attribute::Attribute, Filter, Order, Query};

    #[test]
    fn empty_query() {
        let query: Query = Query::new("testy");
        assert_eq!(query.to_string(), "testy");
    }

    #[test]
    fn limit_query() {
        let mut query: Query = Query::new("testy");
        query.limit = Some(5);
        assert_eq!(query.to_string(), "testy?$top=5");
    }

    #[test]
    fn filter_query() {
        let mut query: Query = Query::new("testy");
        query.filter = Some(Filter::Equal(
            "name",
            Attribute::String(String::from("Testface")),
        ));
        assert_eq!(query.to_string(), "testy?$filter=name eq 'Testface'");
    }

    #[test]
    fn orderby_query() {
        let mut query: Query = Query::new("testy");
        query.order = Some(vec![Order::Ascending("name"), Order::Descending("rank")]);
        assert_eq!(query.to_string(), "testy?$orderby=name asc,rank desc");
    }

    #[test]
    fn full_query() {
        let mut query: Query = Query::new("testy");
        query.limit = Some(5);
        query.filter = Some(Filter::Equal(
            "name",
            Attribute::String(String::from("Testface")),
        ));
        query.order = Some(vec![Order::Ascending("name"), Order::Descending("rank")]);
        assert_eq!(
            query.to_string(),
            "testy?$top=5&$filter=name eq 'Testface'&$orderby=name asc,rank desc"
        );
    }
}
