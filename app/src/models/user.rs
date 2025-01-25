use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistration {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub code: String,
}
