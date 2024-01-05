use wiremock::{Request, Match};

struct SendEmailBodyMatcher;

impl Match for SendEmailBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
        if let Ok(body) = result {
            body.get("From").is_some()
            && body.get("To").is_some()
            && body.get("Subject").is_some()
            && body.get("HtmlBody").is_some()
            && body.get("TextBody").is_some()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod email_client {
    use claim::{assert_ok, assert_err};
    use fake::{faker::{internet::en::SafeEmail, lorem::en::{Sentence, Paragraph}}, Fake, Faker};
    use rusty_krab::{domain::SubscriberEmail, email_client::EmailClient};
    use secrecy::Secret;
    use wiremock::{MockServer, Mock, matchers::{header_exists, header, path, method, any}, ResponseTemplate};

    use crate::SendEmailBodyMatcher;

    // ===== Helper Functions ===============================================
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            Secret::new(Faker.fake()),
            base_url,
            email(),
            std::time::Duration::from_millis(200),
        )
    }
    // ======================================================================

    #[tokio::test]
    async fn send_request_valid() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;
    }

    #[tokio::test]
    async fn send_request_successful() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let response = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        assert_ok!(response);
    }
    
    #[tokio::test]
    async fn send_request_invalid() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let response = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        assert_err!(response);
    }

    #[tokio::test]
    async fn send_request_timeout() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());
        let response = ResponseTemplate::new(200)
            .set_delay(std::time::Duration::from_secs(5));

        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Sentence(1..10).fake();

        let response = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        assert_err!(response);
    }
}
