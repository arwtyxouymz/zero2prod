use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    Form(data): Form<FormData>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        data.email,
        data.name,
        chrono::Utc::now()
    )
    .execute(&pool)
    .await
    .unwrap();

    format!("Welcome {}", data.name)
}
