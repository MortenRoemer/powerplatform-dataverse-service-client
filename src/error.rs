pub struct DataverseError {
    pub message: String,
}

impl DataverseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
