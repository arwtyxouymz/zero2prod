use super::routes::subscribe;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(pool))
}
