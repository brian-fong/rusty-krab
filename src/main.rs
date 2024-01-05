use rusty_krab::configuration::get_config;
use rusty_krab::email_client::EmailClient;
use rusty_krab::startup::start;
use rusty_krab::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    let name = String::from("rusty-krab");
    let filter = String::from("info");
    let subscriber = get_tracing_subscriber(name, filter, std::io::stdout);
    init_tracing_subscriber(subscriber);

    // Read configuration file
    let config = get_config().expect("Failed to read configuration");

    // Build email client
    let sender_email =  config.email_client.sender()
        .expect("Invalid sender email address");
    let timeout = config.email_client.timeout();
    let email_client = EmailClient::new(
        config.email_client.auth_token,
        config.email_client.base_url,
        sender_email,
        timeout,
    );

    // Initialize TCP socket at configured port
    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&addr)?;

    // Initialize database connection
    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());

    // Start server
    tracing::info!("Starting server: [http://{}]...", addr);
    start(listener, pool, email_client)?.await?;

    Ok(())
}
