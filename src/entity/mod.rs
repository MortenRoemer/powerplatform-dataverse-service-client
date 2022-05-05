use std::collections::HashMap;
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
    EntityReferenceCollection(Vec<EntityReference<'a>>),
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