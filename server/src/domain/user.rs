use app::models::{response::RegisterResponse, user::form::RegisterUserForm};
use secrecy::ExposeSecret;

use super::{user_code::UserCode, username::Username};

#[derive(Debug, Clone, Default)]
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

impl From<NewUser> for RegisterResponse {
    fn from(value: NewUser) -> Self {
        Self {
            username: value.username.as_ref().into(),
            code: value.code.expose_secret().into(),
        }
    }
}
