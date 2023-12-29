use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod config;
use config::SERVER_CONFIG;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}


#[tokio::main] // this is the runtime
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind(SERVER_CONFIG.base_url)?
    .run()
    .await
}
