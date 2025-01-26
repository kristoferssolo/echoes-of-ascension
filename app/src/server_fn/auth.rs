use leptos::{prelude::*, server};

use crate::models::{response::RegisterResponse, user::form::RegisterUserForm};

#[server(RegisterUser, "/register")]
pub async fn register_user(form: RegisterUserForm) -> Result<RegisterResponse, ServerFnError> {
    Ok(RegisterResponse {
        username: form.username,
        code: String::new(),
    })
}
