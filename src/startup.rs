use crate::configuration::{get_config, DatabaseSettings};
use crate::email_client::EmailClient;
use crate::routes::{check_health, subscribe};
use crate::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let name = String::from("test");
    let env_filter = String::from("debug");
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

pub fn start(
    listener: TcpListener,
    pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/check_health", web::get().to(check_health))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub async fn start_background() -> TestApp {
    // Initialize tracing
    Lazy::force(&TRACING);

    // Read configuration file
    let mut config = get_config().expect("Failed to read configuration");

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

    // Initialize TCP socket listening on 0 port
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());

    // Create new database (and connect to it)
    config.database.db_name = Uuid::new_v4().to_string();
    let pool = init_db(&config.database).await;

    // Start server
    let server = start(listener, pool.clone(), email_client)
        .expect("Failed to start server");
    tokio::spawn(server);

    TestApp { address, pool }
}

pub async fn init_db(config: &DatabaseSettings) -> PgPool {
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
    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}
