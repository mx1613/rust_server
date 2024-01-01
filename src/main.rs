use std::io;
use std::net::TcpListener;

use rust_server::configuration::get_configuration;
use rust_server::startup::run;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    )
    .to_string();
    let listener =
        TcpListener::bind(address).expect("Failed to bind listener");
    run(listener)?.await
}
