use std::net::TcpListener;

use actix_web::http::StatusCode;
use sqlx::{PgPool};

use rust_server::configuration::get_configuration;
use rust_server::startup::run;

struct TestApp {
    address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server =
        run(listener, db_pool.clone()).expect("Failed to spawn rust server.");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

#[tokio::test]
async fn health_check_happy_path() {
    // Arrange
    let test_app = spawn_app().await;
    let health_check_endpoint = format!("{}/health_check", test_app.address);
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
    let app = spawn_app().await;
    let subscribe_endpoint = format!("{}/subscribe", app.address);
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&subscribe_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=John%20Doe&email=john.doe%40example.com")
        .send()
        .await
        .expect(
            format!("Failed to execute POST request on {}.", app.address)
                .as_str(),
        );
    // Assert
    assert!(response.status().is_success());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "john.doe@example.com");
    assert_eq!(saved.name, "John Doe");
}

#[tokio::test]
async fn subscribe_bad_requests() {
    // Arrange
    let app = spawn_app().await;
    let subscribe_endpoint = format!("{}/subscribe", app.address);
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
                format!(
                    "Failed to execute POST request on {}.",
                    app.address
                )
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
