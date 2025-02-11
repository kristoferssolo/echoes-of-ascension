mod health_check;

use axum::{routing::get, Router};

use crate::startup::AppState;
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
};
pub use health_check::*;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, Span};
use uuid::Uuid;

pub fn route(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .with_state(state)
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
                        uri = %request.uri(),
                        matched_path,
                        request_id=%Uuid::new_v4(),
                    )
                })
                .on_request(|request: &Request<_>, span: &Span| {
                    info!(
                        target: "http_requests",
                        parent: span,
                        method = ?request.method(),
                        uri = %request.uri(),
                        "Incoming request"
                    );
                })
                .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
                    info!(
                        target: "http_responses",
                        parent: span,
                        status = response.status().as_u16(),
                        latency = ?latency,
                        "Outgoing response"
                    );
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {},
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {},
                ),
        )
    // .layer(create_telemetry_layer())
}
