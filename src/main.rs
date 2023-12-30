use actix_web::{web, App, HttpResponse, HttpServer};
mod config;
use config::SERVER_CONFIG;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[tokio::test]
    async fn health_check_returns_ok() {
        // Act
        let response = health_check().await;
        // Assert
        assert_eq!(response.status(), StatusCode::OK);
    }
}

#[tokio::main] // this is the runtime
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(SERVER_CONFIG.base_url)?
        .run()
        .await
}
