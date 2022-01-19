use std::net::TcpListener;

use webapp::startup;

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
async fn resources_returns_201_for_valid_json_data() {
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
    assert!(response.status().as_u16(), 201);
}

pub struct TestApp {
    pub address: String,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    tokio::spawn(async move {
        startup::run(listener).await
    });

    TestApp {
        address
    }
}
