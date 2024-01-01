use std::net::TcpListener;

use actix_web::http::StatusCode;
use sqlx::{Connection, PgConnection};

use rust_server::configuration::get_configuration;
use rust_server::startup::run;

fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to spawn rust server.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_happy_path() {
    // Arrange
    let address = spawn_app();
    let health_check_endpoint = format!("{}/health_check", address);
    // Act
    let client = reqwest::Client::new();
    let response = client
        .get(&health_check_endpoint)
        .send()
        .await
        .expect("Failed to execute GET request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_happy_path() {
    // Arrange
    let address = spawn_app();
    let configuration =
        get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let subscribe_endpoint = format!("{}/subscribe", address);
    let client = reqwest::Client::new();
    // Act
    let response = client
        .post(&subscribe_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=John%20Doe&email=john.doe%40example.com")
        .send()
        .await
        .expect(
            format!("Failed to execute POST request on {}.", address).as_str(),
        );
    // Assert
    assert!(response.status().is_success());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    
    assert_eq!(saved.email, "john.doe@example.com");
    assert_eq!(saved.name, "John Doe");
}

#[tokio::test]
async fn subscribe_bad_requests() {
    // Arrange
    let address = spawn_app();
    let subscribe_endpoint = format!("{}/subscribe", address);
    let test_cases = vec![
        ("name=John%20Doe", "missing the email"),
        ("email=john.doe%40example.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let client = reqwest::Client::new();
        let response = client
            .post(&subscribe_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect(
                format!("Failed to execute POST request on {}.", address)
                    .as_str(),
            );
        // Assert
        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
