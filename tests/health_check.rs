use std::net::TcpListener;

use sqlx::MySqlPool;

use webapp::{
    configuration::get_configuration,
    startup
};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/healthz", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn create_resource_returns_201_for_valid_json_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let json = r#"{
        "resource_id": "places"
    }"#;
    let response = client
        .post(format!("{}/resources", &app.address))
        .json(json)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn create_resource_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("{}", "empty data"),
    ];

    for (invalid_json, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/resources", &app.address))
            .json(invalid_json)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with '400 Bad Request' when the payload were {}",
            error_message
        );
    }
}


use once_cell::sync::Lazy;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});


pub struct TestApp {
    pub address: String,
    pub db_pool: MySqlPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = MySqlPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Mariadb");

    let db_pool = configure_database(&configuration.database).await;
    let db_pool_clone = db_pool.clone();
    tokio::spawn(async move {
        startup::run(listener, db_pool_clone).await
    });

    TestApp {
        address,
        db_pool,
    }
}



async fn configure_database(config: &DatabaseSettings) -> DbPool {
    use sqlx::migrate::MigrateDatabase;

    // Create database
//     let mut connection = PgConnection::connect_with(&config.without_db())
//         .await
//         .expect("Failed to connect");
//     connection
//         .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
//         .await
//         .expect("Failed to create database");

    // Migrate database
    let pool = MySqlPool::connect_with(config.with_test_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations!");
    pool
}
