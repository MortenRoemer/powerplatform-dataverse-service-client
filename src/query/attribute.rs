use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

/**
A Dataverse AttributeValue for use in query filters

Please note that this enum is for use in queries only as it is not serializable
*/
pub enum Attribute {

    /// Indicates a `null` value
    Null,

    /// Indicates a boolean value like `true` or `false`
    Boolean(bool),

    /// Indicates a 64-bit signed integer
    Integer(i64),

    /// Indicates a 64-bit floating decimal number
    Decimal(f64),

    /// Indicates a string of characters
    String(String),

    /// Indicates a date and time expressed as UTC
    DateTime(DateTime<Utc>),

    /// Indicates an Universally Unique Identifier
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
