use app::models::user::error::UserError;
use sqlx::PgPool;

use crate::{domain::user::NewUser, error::user::ServerUserError};

#[tracing::instrument(name = "Saving new user details in the database", skip(new_user, pool))]
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
                ServerUserError::User(UserError::UsernameTaken(new_user.username.as_ref().into()))
            }
            _ => ServerUserError::Database(e),
        }
    })?;
    Ok(())
}
