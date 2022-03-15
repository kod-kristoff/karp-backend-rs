use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use karp_server::{configuration::get_configuration, startup, telemetry};

#[tokio::main]
async fn main() {
    let subscriber = telemetry::get_subscriber("karp".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    // TCP listener
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port,
    );
    let listener = TcpListener::bind(&address).unwrap();
    tracing::info!("listening on {}", address);

    if let Err(e) = startup::run(listener, connection_pool).await {
        eprintln!("Server error: {}", e);
    }
}
