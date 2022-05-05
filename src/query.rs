use std::{fmt::Display};

use crate::entity::AttributeValue;

pub struct Query<'a> {
    pub logical_name: &'a str,
    pub columns: Option<Vec<&'a str>>,
    pub limit: Option<u32>,
    pub filter: Option<Filter<'a>>,
    pub order: Option<Vec<Order<'a>>>,
}

impl<'a> Query<'a> {
    pub fn new(logical_name: &'a str) -> Self {
        Self {
            logical_name,
            columns: None,
            limit: None,
            filter: None,
            order: None,
        }
    }
}

impl<'a> Display for Query<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first_item = true;
        f.write_str(self.logical_name)?;

        if let Some(columns) = &self.columns {
            if first_item {
                f.write_str("?")?;
                first_item = false;
            } else {
                f.write_str("&")?;
            }

            f.write_str("$select=")?;
            let mut first_column = true;

            for column in columns.iter() {
                if first_column {
                    first_column = false;
                } else {
                    f.write_str(",")?;
                }

                f.write_fmt(format_args!("{}", column))?;
            }
        }

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

pub enum Filter<'a> {
    Equal(&'a str, AttributeValue<'a>),
    NotEqual(&'a str, AttributeValue<'a>),
    GreaterThan(&'a str, AttributeValue<'a>),
    GreaterOrEqual(&'a str, AttributeValue<'a>),
    LessThan(&'a str, AttributeValue<'a>),
    LessOrEqual(&'a str, AttributeValue<'a>),
    Contains(&'a str, AttributeValue<'a>),
    StartsWith(&'a str, AttributeValue<'a>),
    EndsWith(&'a str, AttributeValue<'a>),
    And(Box<Filter<'a>>, Box<Filter<'a>>),
    Or(Box<Filter<'a>>, Box<Filter<'a>>),
    Not(Box<Filter<'a>>),
}

impl<'a> Display for Filter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Filter::*;
        match self {
            Equal(name, attribute) => f.write_fmt(format_args!("{} eq {}", name, attribute)),
            NotEqual(name, attribute) => f.write_fmt(format_args!("{} ne {}", name, attribute)),
            GreaterThan(name, attribute) => f.write_fmt(format_args!("{} gt {}", name, attribute)),
            GreaterOrEqual(name, attribute) => f.write_fmt(format_args!("{} ge {}", name, attribute)),
            LessThan(name, attribute) => f.write_fmt(format_args!("{} lt {}", name, attribute)),
            LessOrEqual(name, attribute) => f.write_fmt(format_args!("{} le {}", name, attribute)),
            Contains(name, attribute) => f.write_fmt(format_args!("contains({},{})", name, attribute)),
            StartsWith(name, attribute) => f.write_fmt(format_args!("startswith({},{})", name, attribute)),
            EndsWith(name, attribute) => f.write_fmt(format_args!("endswith({},{})", name, attribute)),
            And(left, right) => f.write_fmt(format_args!("{} and {}", left, right)),
            Or(left, right) => f.write_fmt(format_args!("{} or {}", left, right)),
            Not(subfilter) => f.write_fmt(format_args!("not {}", subfilter)),
        }
    }
}

pub enum Order<'a> {
    Ascending(&'a str),
    Descending(&'a str),
}

impl<'a> Display for Order<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Order::*;
        match self {
            Ascending(name) => f.write_fmt(format_args!("{} asc", name)),
            Descending(name) => f.write_fmt(format_args!("{} desc", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{query::{Query, Filter, Order}, entity::AttributeValue};

    #[test]
    fn empty_query() {
        let query = Query::new("testy");
        assert_eq!(query.to_string(), "testy");
    }

    #[test]
    fn select_query() {
        let mut query = Query::new("testy");
        query.columns = Some(vec!["first", "second"]);
        assert_eq!(query.to_string(), "testy?$select=first,second");
    }

    #[test]
    fn limit_query() {
        let mut query = Query::new("testy");
        query.limit = Some(5);
        assert_eq!(query.to_string(), "testy?$top=5");
    }

    #[test]
    fn filter_query() {
        let mut query = Query::new("testy");
        query.filter = Some(Filter::Equal("name", AttributeValue::Text(String::from("Testface"))));
        assert_eq!(query.to_string(), "testy?$filter=name eq 'Testface'");
    }

    #[test]
    fn orderby_query() {
        let mut query = Query::new("testy");
        query.order = Some(vec![Order::Ascending("name"), Order::Descending("rank")]);
        assert_eq!(query.to_string(), "testy?$orderby=name asc,rank desc");
    }

    #[test]
    fn full_query() {
        let mut query = Query::new("testy");
        query.columns = Some(vec!["first", "second"]);
        query.limit = Some(5);
        query.filter = Some(Filter::Equal("name", AttributeValue::Text(String::from("Testface"))));
        query.order = Some(vec![Order::Ascending("name"), Order::Descending("rank")]);
        assert_eq!(query.to_string(), "testy?$select=first,second&$top=5&$filter=name eq 'Testface'&$orderby=name asc,rank desc");
    }
}
