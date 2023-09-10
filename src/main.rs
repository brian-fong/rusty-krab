use std::net::TcpListener;
use rusty_krab::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    run(listener)?.await
}
