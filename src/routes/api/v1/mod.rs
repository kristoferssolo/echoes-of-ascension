mod auth;

use axum::{routing::post, Router};

use crate::startup::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/register", post(auth::register))
}
