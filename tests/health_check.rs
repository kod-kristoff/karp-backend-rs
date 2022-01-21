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



pub struct TestApp {
    pub address: String,
    pub db_pool: MySqlPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = MySqlPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Mariadb");

    let db_pool_clone = db_pool.clone();
    tokio::spawn(async move {
        startup::run(listener, db_pool_clone).await
    });

    TestApp {
        address,
        db_pool,
    }
}
