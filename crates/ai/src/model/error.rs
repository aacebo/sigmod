#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ModelError {
    Parse(ParseError),
    Http(String),
    Api { status: u16, message: String },
}

impl From<ParseError> for ModelError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<reqwest::Error> for ModelError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value.to_string())
    }
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(v) => write!(f, "{}", v),
            Self::Http(v) => write!(f, "{}", v),
            Self::Api { status, message } => write!(f, "{} {}", status, message),
        }
    }
}

impl std::error::Error for ModelError {}

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
