use axum_zero2prod::configuration::Settings;
use axum_zero2prod::startup::app;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to read configuration.");

    let pool = PgPoolOptions::new()
        .connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect Postgres");

    let addr = SocketAddr::from(([127, 0, 0, 1], settings.application_port));
    println!("The app is listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app(pool).into_make_service())
        .await
        .unwrap()
}
