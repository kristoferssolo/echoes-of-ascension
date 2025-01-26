mod api;
mod health_check;

use app::{shell, App};
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::get,
    Router,
};
use health_check::health_check;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use uuid::Uuid;

use crate::startup::AppState;

pub fn route(state: AppState) -> Router {
    let leptos_options = state.leptos_options.clone();
    let routes = generate_route_list(App);

    let api_router = api::routes().with_state(state.clone());

    Router::new()
        .route("/health_check", get(health_check))
        // API routes with proper nesting
        .nest("/api", api_router)
        // Leptos setup
        .leptos_routes(&leptos_options, routes, {
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            }
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        // Tracing layer
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
