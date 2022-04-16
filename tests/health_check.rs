use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use axum_zero2prod::configuration::{DatabaseSettings, Settings};
use axum_zero2prod::startup::app;
use hyper;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tower::ServiceExt;

#[tokio::test]
async fn health_check_works() {
    let app = get_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(body.is_empty());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let configuration = Settings::new().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let app = get_app().await;
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/subscriptions")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, message) in test_cases {
        let response = get_app()
            .await
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/subscriptions")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(Body::from(invalid_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "The API did not fail with 422 Unprocessable Entity when the payload was {}",
            message
        );
    }
}

pub struct TestApp {
    pub app: Router,
    pub db_pool: PgPool,
}

async fn get_app() -> Router {
    let mut configuration = Settings::new().expect("Failed to read configuration");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();

    let pool = configure_database(&configuration.database).await;

    app(pool)
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
