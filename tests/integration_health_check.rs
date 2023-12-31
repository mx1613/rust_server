use std::net::TcpListener;

#[tokio::test]
async fn health_check_returns_ok() {
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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = rust_server::run(listener).expect("Failed to spawn rust server.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
