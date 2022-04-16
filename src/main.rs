use axum_zero2prod::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("The app is listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap()
}
