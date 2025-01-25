use serde::{Deserialize, Serialize};

use super::{user_code::UserCode, username::Username};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserForm {
    pub username: String,
}

#[derive(Debug, Default)]
pub struct NewUser {
    pub username: Username,
    pub code: UserCode,
}

impl TryFrom<RegisterUserForm> for NewUser {
    type Error = String;
    fn try_from(value: RegisterUserForm) -> Result<Self, Self::Error> {
        let username = value.username.try_into()?;
        Ok(Self {
            username,
            ..Default::default()
        })
    }
}
