use rusty_krab::startup::start_background;

#[tokio::test]
async fn valid_form() {
    // Start server
    let app = start_background().await;

    // Create a subscription
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send POST request");

    // Check for successful creation
    assert_eq!(200, response.status().as_u16());

    // Read subscription from database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription");

    // Check subscription
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn invalid_form() {
    // Start server
    let app = start_background().await;

    // Assign test cases
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    // (Attempt to) Create subscriptions
    let client = reqwest::Client::new();
    for (body, err_msg) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send POST request");

        // Check for unsuccesful creation
        assert_eq!(400, response.status().as_u16(), "{}", err_msg);
    }
}
