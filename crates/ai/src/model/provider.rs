#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderId {
    OpenAI,
    Anthropic,
    HuggingFace,
    #[serde(untagged)]
    Other(String),
}

impl ProviderId {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
            Self::HuggingFace => "huggingface",
            Self::Other(other) => other.as_str(),
        }
    }
}

impl From<String> for ProviderId {
    fn from(s: String) -> Self {
        match s.as_str() {
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
