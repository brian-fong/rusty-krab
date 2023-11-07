use crate::configuration::{get_config, DatabaseSettings};
use crate::routes::{check_health, subscribe};
use crate::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
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

pub fn start(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/check_health", web::get().to(check_health))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub async fn start_background() -> TestApp {
    // Initialize tracing
    Lazy::force(&TRACING);

    // Assign TCP socket to 0 port
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());

    // Create new database (and connect to it)
    let mut config = get_config().expect("Failed to read configuration");
    config.database.db_name = Uuid::new_v4().to_string();
    let pool = init_db(&config.database).await;

    // Start server
    let server = start(listener, pool.clone()).expect("Failed to start server");
    let _ = tokio::spawn(server);

    TestApp { address, pool }
}

pub async fn init_db(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.cstring_alt())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    // Migrate database
    let pool = PgPool::connect(&config.cstring())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}
