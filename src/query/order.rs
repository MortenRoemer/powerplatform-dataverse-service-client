use std::fmt::Display;

pub enum Order {
    Ascending(&'static str),
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
