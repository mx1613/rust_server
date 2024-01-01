use std::io;
use std::net::TcpListener;

use rust_server::configuration::SERVER_CONFIG;
use rust_server::startup::run;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind(SERVER_CONFIG.base_url)
        .expect("Failed to bind listener");
    run(listener)?.await
}
