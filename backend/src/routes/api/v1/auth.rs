use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{
    db::users::{insert_user, ServerUserError},
    domain::user::{error::UserError, new_user::NewUser},
    error::app::AppError,
    startup::AppState,
};

#[derive(Debug, Deserialize)]
pub struct FormData {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub username: String,
    pub code: String,
}

#[tracing::instrument(
    name = "Creating new user",
    skip(state, payload),
    fields(
        username= %payload.username,
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let new_user = payload
        .try_into()
        .map_err(|e: UserError| AppError::Validation(e.to_string()))?;

    match insert_user(&state.pool, &new_user).await {
        Ok(()) => Ok((StatusCode::CREATED, Json(Response::from(new_user)))),
        Err(ServerUserError::User(UserError::UsernameTaken(username))) => {
            Err(AppError::AlreadyExists {
                resource: "User",
                id: username,
            })
        }
        Err(e) => {
            tracing::error!("Failed to register user: {}", e);
            Err(AppError::Internal(anyhow!(e)))
        }
    }
}

impl TryFrom<FormData> for NewUser {
    type Error = UserError;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let username = value.username.try_into()?;
        Ok(Self {
            username,
            ..Default::default()
        })
    }
}

impl From<NewUser> for Response {
    fn from(value: NewUser) -> Self {
        Self {
            username: value.username.into(),
            code: value.code.expose_secret().into(),
        }
    }
}
