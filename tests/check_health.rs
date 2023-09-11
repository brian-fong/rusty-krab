use rusty_krab::start;
use std::net::TcpListener;

#[tokio::test]
async fn check_health() {
    let address = start_background();

    let  client = reqwest::Client::new();

    let response = client
        .get(format!("{}/checkhealth", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn start_background() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = start(listener).expect("Failed to bind address");
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
