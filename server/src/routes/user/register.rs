use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use secrecy::ExposeSecret;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;

use crate::{domain::new_user::NewUser, startup::AppState};

#[derive(Deserialize)]
pub struct FormData {
    username: String,
}

pub async fn register(
    State(state): State<AppState>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let new_user = match form.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    if insert_user(&state.pool, &new_user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    todo!()
}

#[tracing::instrument(name = "Saving new user details in the database", skip(new_user, pool))]
pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO "user" (username, code)
            VALUES ($1, $2)
        "#,
        new_user.username.as_ref(),
        new_user.code.expose_secret()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

impl TryFrom<FormData> for NewUser {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let username = value.username.try_into()?;
        Ok(Self {
            username,
            ..Default::default()
        })
    }
}
