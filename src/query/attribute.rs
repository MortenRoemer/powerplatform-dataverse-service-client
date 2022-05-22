use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum Attribute {
    Null,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(String),
    DateTime(DateTime<Utc>),
    Uuid(Uuid),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Null => f.write_str("null"),
            Attribute::Boolean(value) => f.write_fmt(format_args!("{}", value)),
            Attribute::Integer(value) => f.write_fmt(format_args!("{}", value)),
            Attribute::Decimal(value) => f.write_fmt(format_args!("{}", value)),
            Attribute::String(value) => f.write_fmt(format_args!("'{}'", value)),
            Attribute::DateTime(value) => f.write_fmt(format_args!("'{}'", value)),
            Attribute::Uuid(value) => f.write_fmt(format_args!("'{}'", value.as_hyphenated())),
        }
    }
}
