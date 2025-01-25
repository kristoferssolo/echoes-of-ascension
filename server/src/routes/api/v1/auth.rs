use app::db::users::insert_user;
use app::models::{
    response::{ErrorResponse, RegisterResponse},
    user::{error::UserError, new_user::RegisterUserForm},
};
use app::startup::AppState;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
#[tracing::instrument(
    name = "Creating new user",
    skip(data, state),
    fields(
        username= %data.username,
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(data): Json<RegisterUserForm>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let new_user = data
        .try_into()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ErrorResponse::from(e))))?;

    match insert_user(&state.pool, &new_user).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(RegisterResponse::from(new_user)))),
        Err(UserError::UsernameTaken(username)) => Err((
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
