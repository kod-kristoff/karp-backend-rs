use std::net::TcpListener;

use axum::{extract::Extension, Router};
use sqlx::PgPool;

use crate::routes;

pub async fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<()> {
    println!("webapp::startup::run()");
    use axum::routing::{get, post};
    let app = Router::new()
        .route("/healthz", get(routes::health_check))
        .route("/resources", post(routes::create_resource))
        .layer(Extension(db_pool));
    axum::Server::from_tcp(listener)
        .expect("Failed binding")
        .serve(app.into_make_service())
        .await
        .expect("Server error");
    Ok(())
}
