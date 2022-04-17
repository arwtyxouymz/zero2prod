use super::routes::subscribe;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Extension, Router,
};
use hyper::{Body, Request};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(pool))
        .layer(
            TraceLayer::new_for_http().make_span_with(|_request: &Request<Body>| {
                let request_id = uuid::Uuid::new_v4().to_string();
                tracing::info_span!("request_id", %request_id)
            }),
        )
}
