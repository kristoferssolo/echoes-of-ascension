use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub username: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl<T> From<T> for ErrorResponse
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            error: value.into(),
        }
    }
}
