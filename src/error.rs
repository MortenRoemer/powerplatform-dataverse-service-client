use std::fmt::Display;


#[derive(Debug)]
pub struct DataverseError {
    pub message: String,
}

impl DataverseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for DataverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
