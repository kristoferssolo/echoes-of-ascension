use sqlx::PgPool;

use crate::models::user::{error::UserError, new_user::NewUser};

#[tracing::instrument(name = "Saving new user details in the database", skip(new_user, pool))]
pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<(), UserError> {
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
                UserError::UsernameTaken(new_user.username.as_ref().to_string())
            }
            _ => UserError::Database(e),
        }
    })?;
    Ok(())
}
