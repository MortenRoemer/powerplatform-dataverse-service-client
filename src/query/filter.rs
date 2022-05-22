use std::fmt::Display;

use super::attribute::Attribute;

pub enum Filter {
    Equal(&'static str, Attribute),
    NotEqual(&'static str, Attribute),
    GreaterThan(&'static str, Attribute),
    GreaterOrEqual(&'static str, Attribute),
    LessThan(&'static str, Attribute),
    LessOrEqual(&'static str, Attribute),
    Contains(&'static str, Attribute),
    StartsWith(&'static str, Attribute),
    EndsWith(&'static str, Attribute),
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
    Not(Box<Filter>),
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
