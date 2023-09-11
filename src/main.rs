use std::net::TcpListener;
use rusty_krab::start;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    println!("Starting server: [http://{}]...", addr);
    start(listener)?.await
}
