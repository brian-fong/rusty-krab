use rusty_krab::configuration::get_config;
use rusty_krab::startup::Application;
use rusty_krab::telemetry::{get_tracing_subscriber, init_tracing_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let name = String::from("rusty-krab");
    let filter = String::from("info");
    let subscriber = get_tracing_subscriber(name, filter, std::io::stdout);
    init_tracing_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration");
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;

    Ok(())
}
