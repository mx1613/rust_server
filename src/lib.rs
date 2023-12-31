use std::io;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub mod config;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// I guess this would be the equivalend of a schema in FastAPI
#[derive(serde::Deserialize)] // this will add the Deserialize trait to the struct
pub struct FormData {
    email: String,
    name: String,
}
// and we add the schema to the handler, actix-web will deserialize the request body to the schema
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
