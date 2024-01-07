use std::io;
use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use rust_server::configuration::get_configuration;
use rust_server::startup::run;
use rust_server::telemetry::{get_tracer, init_tracing};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let tracer =
        get_tracer("rust_server".into(), "info".into(), std::io::stdout);
    init_tracing(tracer);

    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    )
    .to_string();
    let listener = TcpListener::bind(address).expect("Failed to bind listener");
    let db_pool = PgPool::connect(
        configuration.database.connection_string().expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres.");

    run(listener, db_pool)?.await?;
    Ok(())
}
