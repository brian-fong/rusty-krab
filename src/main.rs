use rusty_krab::configuration::get_config;
use rusty_krab::startup::start;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read configuration
    let config = get_config().expect("Failed to read configuration");

    // Assign TCP socket
    let addr = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(&addr)?;

    // Connect to Postgres database
    let pool = PgPool::connect(&config.database.cstring())
        .await
        .expect("Failed to connect to Postgres");

    // Start server
    println!("Starting server: [http://{}]...", addr);
    start(listener, pool)?.await
}
