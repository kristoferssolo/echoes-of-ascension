use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use super::user::new_user::NewUser;

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

impl From<NewUser> for RegisterResponse {
    fn from(value: NewUser) -> Self {
        Self {
            username: value.username.as_ref().to_string(),
            code: value.code.expose_secret().to_string(),
        }
    }
}
