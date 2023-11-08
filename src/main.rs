use rusty_krab::configuration::get_config;
use rusty_krab::startup::start;
use rusty_krab::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let name = String::from("rusty-krab");
    let filter = String::from("info");
    let subscriber = get_tracing_subscriber(name, filter, std::io::stdout);
    init_tracing_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration");

    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&addr)?;

    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&config.database.cstring().expose_secret())
        .expect("Failed to connect to Postgres");

    tracing::info!("Starting server: [http://{}]...", addr);
    start(listener, pool)?.await
}
