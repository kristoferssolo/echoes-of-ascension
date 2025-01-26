use app::models::{
    response::{ErrorResponse, RegisterResponse},
    user::{error::UserError, form::RegisterUserForm},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{db::users::insert_user, error::user::ServerUserError, startup::AppState};

#[tracing::instrument(
    name = "Creating new user",
    skip(payload, state),
    fields(
        username= %payload.username,
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserForm>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let new_user = payload
        .try_into()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ErrorResponse::from(e))))?;

    match insert_user(&state.pool, &new_user).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(RegisterResponse::from(new_user)))),
        Err(ServerUserError::User(UserError::UsernameTaken(username))) => Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse::from(format!(
                "Username {} is already taken.",
                username
            ))),
        )),
        Err(e) => {
            tracing::error!("Failed to register user: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::from("Internal server error")),
            ))
        }
    }
}
