#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParseError {
    subject: String,
}

impl ParseError {
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
        }
    }
}

impl<T: Into<String>> From<T> for ParseError {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid model id '{}'", self.subject)
    }
}
