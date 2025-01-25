mod auth;

use app::startup::AppState;
use axum::{routing::post, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/register", post(auth::register))
}
