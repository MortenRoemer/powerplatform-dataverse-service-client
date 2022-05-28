/**
trait for acquiring the relevant attribute names for queries
*/
pub trait Select {
    /// gets a vector of attribute names that shall be included in
    /// the query select statement
    fn get_columns() -> &'static [&'static str];
}
