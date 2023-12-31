use std::io;
use std::net::TcpListener;

use rust_server::config::SERVER_CONFIG;
use rust_server::run;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind(SERVER_CONFIG.base_url).expect("Failed to bind ls");
    run(listener)?.await
}
