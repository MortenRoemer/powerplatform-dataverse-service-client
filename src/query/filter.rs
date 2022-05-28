use std::fmt::Display;

use super::attribute::Attribute;

/**
Represents a filter for Microsoft Dataverse queries

Filters can be combined with logical operators to form more complex
filter expressions

# Examples
```rust
// example filter for attributes "firstname" and "lastname"
let filter = Filter::Equal("firstname", AttributeValue::String(String::from("Testy")))
    .and(Filter::EndsWith("lastname", AttributeValue::String(String::from("face"))))
```
*/
pub enum Filter {

    /// Indicates an equal `==` expression
    Equal(&'static str, Attribute),

    /// Indicates a not equal `!=` expression
    NotEqual(&'static str, Attribute),

    /// Indicates a greater than `>` expression
    GreaterThan(&'static str, Attribute),

    /// Indicates a greater than or equal `>=` expression
    GreaterOrEqual(&'static str, Attribute),

    /// Indicates a less than `<` expression
    LessThan(&'static str, Attribute),

    /// Indicates a less than or equal `<=` expression
    LessOrEqual(&'static str, Attribute),

    /// Indicates a contains expression as in string containing another string
    Contains(&'static str, Attribute),

    /// Indicates a starts with expression as in a string starts with the content of another string
    StartsWith(&'static str, Attribute),

    /// Indicates an "ends with" expression as in a string ends with the content of another string
    EndsWith(&'static str, Attribute),

    /// Indicates a logical and `&` expression
    And(Box<Filter>, Box<Filter>),

    /// Indicates a logical or `|` expression
    Or(Box<Filter>, Box<Filter>),

    /// Indicates a logical not `!` expression
    Not(Box<Filter>),
}

impl Filter {

    /// Logically combines this filter and the given filter with an `&` expression
    pub fn and(self, other: Filter) -> Self {
        Filter::And(Box::new(self), Box::new(other))
    }

    /// Logically combines this filter and the given filter with an `&` expression
    /// and negates it with a `!` expression
    pub fn not_and(self, other: Filter) -> Self {
        Filter::Not(Box::new(Filter::And(Box::new(self), Box::new(other))))
    }

    /// Logically combines this filter and the given filter with an `|` expression
    pub fn or(self, other: Filter) -> Self {
        Filter::Or(Box::new(self), Box::new(other))
    }

    /// Logically combines this filter and the given filter with an `|` expression
    /// and negates it with a `!` expression
    pub fn not_or(self, other: Filter) -> Self {
        Filter::Not(Box::new(Filter::Or(Box::new(self), Box::new(other))))
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Filter::*;
        match self {
            Equal(name, attribute) => f.write_fmt(format_args!("{} eq {}", name, attribute)),
            NotEqual(name, attribute) => f.write_fmt(format_args!("{} ne {}", name, attribute)),
            GreaterThan(name, attribute) => f.write_fmt(format_args!("{} gt {}", name, attribute)),
            GreaterOrEqual(name, attribute) => {
                f.write_fmt(format_args!("{} ge {}", name, attribute))
            }
            LessThan(name, attribute) => f.write_fmt(format_args!("{} lt {}", name, attribute)),
            LessOrEqual(name, attribute) => f.write_fmt(format_args!("{} le {}", name, attribute)),
            Contains(name, attribute) => {
                f.write_fmt(format_args!("contains({},{})", name, attribute))
            }
            StartsWith(name, attribute) => {
                f.write_fmt(format_args!("startswith({},{})", name, attribute))
            }
            EndsWith(name, attribute) => {
                f.write_fmt(format_args!("endswith({},{})", name, attribute))
            }
            And(left, right) => f.write_fmt(format_args!("{} and {}", left, right)),
            Or(left, right) => f.write_fmt(format_args!("{} or {}", left, right)),
            Not(subfilter) => f.write_fmt(format_args!("not {}", subfilter)),
        }
    }
}
