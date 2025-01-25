mod health_check;
mod user;
use std::time::Duration;

use app::{shell, App};
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::{get, post},
    Router,
};
use health_check::health_check;

use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use user::register;
use uuid::Uuid;

use crate::startup::AppState;

pub fn route(state: AppState) -> Router {
    Router::new()
        .merge(leptos_routes(state.clone()))
        .merge(api_routes(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                    info_span!(
                        "http-request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                        request_id=%Uuid::new_v4(),
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {})
                .on_response(|_response: &Response<_>, _latency: Duration, _span: &Span| {})
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {},
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {},
                ),
        )
}

fn leptos_routes(state: AppState) -> Router {
    let leptos_options = state.leptos_options.clone();
    let routes = generate_route_list(App);

    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
}

fn api_routes(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/health_check", get(health_check))
                .route("/register", post(register)),
        )
        .with_state(state)
}
