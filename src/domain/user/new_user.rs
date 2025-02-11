use super::{user_code::UserCode, username::Username};

#[derive(Debug, Clone, Default)]
pub struct NewUser {
    pub username: Username,
    pub code: UserCode,
}
