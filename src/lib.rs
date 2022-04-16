use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn greet(name: Option<Path<String>>) -> String {
    let name = match name {
        Some(Path(name)) => name,
        None => "World".to_string(),
    };
    format!("Hello {}!", &name)
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/:name", get(greet))
}
