#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(from = "String", into = "String")]
pub enum ProviderId {
    Local,
    OpenAI,
    Anthropic,
    HuggingFace,
    #[serde(untagged)]
    Other(String),
}

impl ProviderId {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Local => "local",
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
            Self::HuggingFace => "huggingface",
            Self::Other(other) => other.as_str(),
        }
    }
}

impl From<ProviderId> for String {
    fn from(value: ProviderId) -> Self {
        value.to_string()
    }
}

impl From<String> for ProviderId {
    fn from(s: String) -> Self {
        match s.as_str() {
            "local" => Self::Local,
            "openai" => Self::OpenAI,
            "anthropic" => Self::Anthropic,
            "huggingface" => Self::HuggingFace,
            _ => Self::Other(s),
        }
    }
}

impl std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
