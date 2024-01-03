use std::io;
use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;

use rust_server::configuration::get_configuration;
use rust_server::startup::run;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    )
    .to_string();
    let listener = TcpListener::bind(address).expect("Failed to bind listener");
    
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, db_pool)?.await
}
