use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Field cannot be empty: {0}")]
    Empty(String),

    #[error("Field too short: {0} (minimum {1} characters)")]
    TooShort(String, usize),

    #[error("Field too long: {0} (maximum {1} characters)")]
    TooLong(String, usize),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}
