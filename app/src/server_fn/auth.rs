use crate::db::users::insert_user;
use crate::models::user::error::UserError;
use crate::models::user::new_user::RegisterUserForm;
use crate::{models::response::RegisterResponse, startup::AppState};
use leptos::{prelude::*, server};

#[server(RegisterUser, "/api/v1/users")]
pub async fn register_user(username: String) -> Result<RegisterResponse, ServerFnError<String>> {
    let state = use_context::<AppState>()
        .ok_or_else(|| ServerFnError::ServerError("AppState not found".into()))?;

    let form = RegisterUserForm { username };
    let new_user = form.try_into().map_err(|e| ServerFnError::ServerError(e))?;

    match insert_user(&state.pool, &new_user).await {
        Ok(_) => Ok(RegisterResponse::from(new_user)),
        Err(UserError::UsernameTaken(username)) => Err(ServerFnError::ServerError(format!(
            "Username {} is already taken",
            username
        ))),
        Err(e) => {
            tracing::error!("Failed to register user: {}", e);
            Err(ServerFnError::ServerError(
                "Internal server error".to_string(),
            ))
        }
    }
}
