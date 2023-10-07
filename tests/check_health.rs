use rusty_krab::startup::start_background;

#[tokio::test]
async fn check_health() {
    let client = reqwest::Client::new();
    let app = start_background().await;

    let response = client
        .get(format!("{}/check_health", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
