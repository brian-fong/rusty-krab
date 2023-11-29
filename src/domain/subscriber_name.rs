use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<SubscriberName, String> {
        // Check if string is empty
        let is_empty_whitespace = name.trim().is_empty();

        // Check if string is too long (more than 256 characters)
        let is_too_long = name.grapheme_indices(true).count() > 256;

        // Check if string contains forbidden characters
        let forbidden_chars = ['/', '"', '\\', '(', ')', '{', '}', '<', '>'];
        let is_forbidden = name.chars().any(|char| forbidden_chars.contains(&char));

        // If no conditions are violated, then return SubscriberName, else error
        if !(is_empty_whitespace || is_too_long || is_forbidden) {
            Ok(Self(name))
        } else {
            Err(format!("Invalid subscriber name: {}", name))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
