use std::fmt;

#[derive(Debug, Clone)]
pub struct SpinnerError {
    message: String,
}

impl SpinnerError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl From<&str> for SpinnerError {
    fn from(error_message: &str) -> Self {
        SpinnerError::new(error_message)
    }
}

impl fmt::Display for SpinnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SpinnerError {}

pub type SpinnerResult<T> = std::result::Result<T, SpinnerError>;
