#[tokio::test]
async fn health_check_returns_ok() {
    // Arrange
    spawn_app();
    // Act
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() {
    let server = rust_server::run().expect("Failed to bind address.");
    let _ = tokio::spawn(server);
}
