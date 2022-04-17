use axum::{
    extract::{Extension, Form},
    http::StatusCode,
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

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(data, pool),
    fields(
        subscriber_eamil = %data.email,
        subscriber_name = %data.name
    )
)]
pub async fn subscribe(
    Form(data): Form<FormData>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    match insert_subscriber(&pool, &data).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pool, data)
)]
pub async fn insert_subscriber(pool: &PgPool, data: &FormData) -> Result<(), sqlx::Error> {
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
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
