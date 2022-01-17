use std::net::{SocketAddr, TcpListener};

use webapp::{
    configuration::get_configuration,
    startup
};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    let listener = TcpListener::bind(&addr).unwrap();
    tracing::info!("listening on {}", addr);
    if let Err(e) = startup::run(listener).await {
        eprintln!("Server error: {}", e);
    }
}
