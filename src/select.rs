pub trait Select {
    fn get_columns() -> &'static [&'static str];

    fn empty() -> EmptySelect {
        EmptySelect {}
    }
}

pub struct EmptySelect {}

impl Select for EmptySelect {
    fn get_columns() -> &'static [&'static str] {
        &[]
    }
}
