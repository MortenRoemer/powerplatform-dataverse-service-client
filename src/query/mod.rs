use std::fmt::Display;

use self::{filter::Filter, order::Order};

pub mod attribute;
pub mod filter;
pub mod order;

pub struct Query {
    pub logical_name: &'static str,
    pub limit: Option<u32>,
    pub filter: Option<Filter>,
    pub order: Option<Vec<Order>>,
}

impl Query {
    pub fn new(logical_name: &'static str) -> Self {
        Self {
            logical_name,
            limit: None,
            filter: None,
            order: None,
        }
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
