use once_cell::sync::Lazy;
use rusty_krab::{
    configuration::{get_config, DatabaseSettings},
    startup::{get_pool, Application},
    telemetry::{get_tracing_subscriber, init_tracing_subscriber},
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let name = "test".to_string();
    let env_filter = "debug".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_tracing_subscriber(name, env_filter, std::io::stdout);
        init_tracing_subscriber(subscriber);
    } else {
        let subscriber = get_tracing_subscriber(name, env_filter, std::io::sink);
        init_tracing_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request")
    }
}

pub async fn start_background() -> TestApp {
    Lazy::force(&TRACING);

    // Read configuration, randomizing database name and application port
    let config = {
        let mut config_0 = get_config().expect("Failed to read configuration");
        config_0.database.db_name = Uuid::new_v4().to_string();
        config_0.application.port = 0;
        config_0
    };

    // Create and migrate database
    init_db(&config.database).await;

    // Build application
    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application");

    // Construct TestApp fields
    let address = format!("http://127.0.0.1:{}", application.port());
    let pool = get_pool(&config.database);

    // Start server
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp { address, pool }
}

async fn init_db(config: &DatabaseSettings) -> PgPool {
    // Connect to postgres
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    // Create database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    // Connect to database
    let pool = PgPool::connect_with(config.with_db()).await.expect("Failed to connect to Postgres");

    // Run database migrations
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate database");

    pool
}
