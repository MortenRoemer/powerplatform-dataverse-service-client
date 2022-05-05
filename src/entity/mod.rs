use std::{collections::HashMap, fmt::Display};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Entity<'a> {
    pub logical_name: &'a str,
    pub id: Uuid,
    pub attributes: HashMap<&'a str, Option<AttributeValue<'a>>>,
}

impl<'a> Entity<'a> {
    pub fn new(logical_name: &'a str, id: Uuid) -> Self {
        Self {
            logical_name,
            id,
            attributes: HashMap::new(),
        }
    }

    pub fn with_attributes(logical_name: &'a str, id: Uuid, attributes: HashMap<&'a str, Option<AttributeValue<'a>>>) -> Self {
        Self {
            logical_name,
            id,
            attributes,
        }
    }
}

pub enum AttributeValue<'a> {
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    Text(String),
    DateTime(DateTime<Utc>),
    EntityReference(EntityReference<'a>),
}

impl<'a> Display for AttributeValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AttributeValue::*;
        match self {
            Boolean(value) => f.write_fmt(format_args!("{}", value)),
            Integer(value) => f.write_fmt(format_args!("{}", value)),
            Decimal(value) => f.write_fmt(format_args!("{}", value)),
            Text(value) => f.write_fmt(format_args!("'{}'", value)),
            DateTime(value) => f.write_fmt(format_args!("{}", value)),
            EntityReference(value) => f.write_fmt(format_args!("{}", value)),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct EntityReference<'a> {
    pub logical_name: &'a str,
    pub id: Uuid,
}

impl<'a> EntityReference<'a> {
    pub fn new(logical_name: &'a str, id: Uuid) -> Self {
        Self {
            logical_name,
            id,
        }
    }
}

impl<'a> Display for EntityReference<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.id))
    }
}
