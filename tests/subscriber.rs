#[cfg(test)]
mod subscriber_name {
    use claim::{assert_err, assert_ok};
    use rusty_krab::domain::SubscriberName;

    #[test]
    fn name_valid() {
        let name = "Brian".to_string();
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn name_empty() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn name_whitespace_only() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn name_256_chars() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn name_more_than_256_chars() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn name_invalid_chars() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
}

mod subscriber_email {
    use claim::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use rusty_krab::domain::SubscriberEmail;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn emails_valid(email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(email.0).is_ok()
    }

    #[test]
    fn email_empty() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol() {
        let email = "0xfriangmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject() {
        let email = "@gmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_domain() {
        let email = "0xfrian@".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
