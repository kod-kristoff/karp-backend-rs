use std::net::TcpListener;

use axum::Router;

use crate::routes;

pub async fn run(
    listener: TcpListener,
) -> std::io::Result<()> {
    println!("webapp::startup::run()");
    use axum::routing::{get, post};
    let app = Router::new()
        .route("/healthz", get(routes::health_check))
        ;
    axum::Server::from_tcp(listener)
        .expect("Failed binding")
        .serve(app.into_make_service())
        .await
        .expect("Server error");
    Ok(())
}
