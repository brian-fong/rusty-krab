use rusty_krab::configuration::get_config;
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

    // Initialize TCP socket at configured port
    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&addr)?;

    // Initialize database connection
    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());

    // Start server
    tracing::info!("Starting server: [http://{}]...", addr);
    start(listener, pool)?.await?;

    Ok(())
}

// use resend_client_rs::{emails::SendEmailRequest, Client};
//
// #[tokio::main]
// async fn main() {
//     println!("Sending email...");
//     let client = Client::new("re_Y2SopCQo_FwbPFpTkWw5KXFLMxWo4a72m");
//     let response = client
//         .email_service
//         .send(&SendEmailRequest {
//             subject: "hello world!".to_string(),
//             from: "frian <hello@brian-fong.com>".to_string(),
//             to: vec!["168jonathan@gmail.com".to_string()],
//             cc: None,
//             bcc: None,
//             reply_to: None,
//             html: Some(
//                 "<p>this email was sent from a resend client written in rust!</p>".to_string(),
//             ),
//             text: Some("this email was sent from a resend client written in rust!".to_string()),
//             tags: None,
//             attachments: None,
//             headers: None,
//         })
//         .await;
//     println!("Response: {:#?}", response);
// }
