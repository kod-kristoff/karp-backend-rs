use std::net::TcpListener;

use sqlx::MySqlPool;

use webapp::{
    configuration::get_configuration,
    startup
};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_pool = MySqlPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Mariadb.");
    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&addr).unwrap();
    tracing::info!("listening on {}", addr);
    if let Err(e) = startup::run(listener, connection_pool).await {
        eprintln!("Server error: {}", e);
    }
}
