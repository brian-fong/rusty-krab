use wiremock::{Mock, ResponseTemplate, matchers::{method, path}};

use crate::helpers::start_background;

#[tokio::test]
async fn return_200_on_valid_fields() {
    let app = start_background().await;
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    let body = "name=Brian%20Fong&email=0xfrian%40gmail.com";
    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscriber_saved_on_database() {
    let app = start_background().await;
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    let body = "name=Brian%20Fong&email=0xfrian%40gmail.com";
    app.post_subscriptions(body.into()).await;
    let data = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(data.email, "0xfrian@gmail.com");
    assert_eq!(data.name, "Brian Fong");
    assert_eq!(data.status, "pending_confirmation");
}

#[tokio::test]
async fn return_400_on_missing_fields() {
    let app = start_background().await;

    let test_cases = vec![
        ("name=Brian%20Fong", "missing email"),
        ("email=0xfrian%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];
    for (body, err_msg) in test_cases {
        let response = app.post_subscriptions(body.into()).await;
        assert_eq!(400, response.status().as_u16(), "{}", err_msg);
    }
}

#[tokio::test]
async fn return_400_on_empty_fields() {
    let app = start_background().await;

    let test_cases = vec![
        ("name=&email=0xfrian%40gmail.com", "empty name"),
        ("name=Brian&20Fong&email=", "empty email"),
        ("name=jonathan&email=sus-email", "invalid email"),
    ];
    for (body, err_msg) in test_cases {
        let response = app.post_subscriptions(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "Test Case: {}",
            err_msg
        );
    }
}

#[tokio::test]
async fn confirmation_sent_on_success() {
    let app = start_background().await;
    let body = "name=Brian%20Fong&email=0xfrian%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    
    app.post_subscriptions(body.into()).await;
}

#[tokio::test]
async fn confirmation_contains_link() {
    let app = start_background().await;
    let body = "name=Brian%20Fong&email=0xfrian%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    assert_eq!(confirmation_links.html, confirmation_links.text);
}
