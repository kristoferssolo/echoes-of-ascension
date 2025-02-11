use sqlx::PgPool;
use thiserror::Error;

use crate::{domain::user::new_user::NewUser, errors::user::UserError};

#[derive(Debug, Error)]
pub enum ServerUserError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Database error: {0}")]
    User(#[from] UserError),
}

#[tracing::instrument(name = "Saving new user details in the database", skip(pool, new_user))]
pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<(), ServerUserError> {
    sqlx::query!(
        r#"
        INSERT INTO "user" (username, code)
            VALUES ($1, $2)
        "#,
        new_user.username.as_ref(),
        new_user.code.hash()?
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        match e {
            sqlx::Error::Database(ref dbe) if dbe.constraint() == Some("user_username_key") => {
                ServerUserError::User(UserError::UsernameTaken(new_user.username.to_string()))
            }
            _ => ServerUserError::Database(e),
        }
    })?;
    Ok(())
}
