use axum_zero2prod::configuration::Settings;
use axum_zero2prod::startup::app;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to read configuration.");

    let env_filter = EnvFilter::new(
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "axum_zero2prod=debug,tower_http=debug".into()),
    );

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&settings.database.connection_string().expose_secret())
        .expect("Failed to connect Postgres");

    let addr = SocketAddr::from((settings.application.host, settings.application.port));
    tracing::info!("The app is listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app(pool).into_make_service())
        .await
        .unwrap()
}
