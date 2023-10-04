use rusty_krab::startup::start_background;


#[tokio::test]
async fn check_health() {
    let addr = start_background();
    let route = format!("{}/check_health", addr);

    let client = reqwest::Client::new();

    let response = client.get(route).send().await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn valid_form() {
    let addr = start_background();
    let route = format!("{}/subscriptions", addr);
    
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client.post(route)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body).send().await
        .expect("Failed to send POST request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn invalid_form() {
    let addr = start_background();
    let route = format!("{}/subscriptions", addr);
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, err_msg) in test_cases {
        let response = client.post(&route)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body).send().await
            .expect("Failed to send POST request");
        assert_eq!(400, response.status().as_u16(), "{}", err_msg);
    }
}
