use crate::helpers::start_background;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn return_200_on_valid_token() {
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
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn return_400_on_invalid_token() {
    let app = start_background().await;
    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn updates_status_on_confirmation() {
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

    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let data = sqlx::query!("SELECT name, email, status FROM subscriptions")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to read subscription");

    assert_eq!(data.name, "Brian Fong");
    assert_eq!(data.email, "0xfrian@gmail.com");
    assert_eq!(data.status, "confirmed");
}
