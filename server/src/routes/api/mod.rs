mod v1;

use app::startup::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1", v1::routes())
}
