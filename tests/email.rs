#[cfg(test)]
mod email {
    use fake::{
        faker::lorem::en::{Paragraph, Sentence},
        Fake,
    };
    use resend_client_rs::{emails::SendEmailRequest, reqlib::APIResponse, Client};

    #[tokio::test]
    async fn send_email() {
        // Load environmental variables
        dotenv::dotenv().ok();
        let api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY undefined");
        let resend_client = Client::new(api_key);

        // Build email
        let from = "frian <hi@brian-fong.com>".to_string();
        let to = vec!["0xfrian@gmail.com".to_string()];
        let subject = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Send email
        let send_result = resend_client
            .email_service
            .send(&SendEmailRequest {
                from,
                to,
                subject,
                text: Some(content),
                cc: None,
                bcc: None,
                reply_to: None,
                html: None,
                tags: None,
                attachments: None,
                headers: None,
            })
            .await;

        assert!(send_result.is_ok(), "Problem sending email");

        let email_id: String;
        match send_result.unwrap() {
            APIResponse::Success(email) => email_id = email.id,
            error => panic!("API Error: {:#?}", error),
        }

        let read_result = resend_client.email_service.get(email_id).await;
        assert!(&read_result.is_ok(), "Problem reading email: {:#?}", read_result);
    }
}
