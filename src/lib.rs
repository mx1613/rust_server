use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

mod config;
use config::SERVER_CONFIG;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(SERVER_CONFIG.base_url)?
        .run();

    Ok(server)
}
