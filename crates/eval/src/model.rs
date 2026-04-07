use std::str::FromStr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Local,
    OpenAI,
    Anthropic,
    HuggingFace,
    #[serde(untagged)]
    Other(String),
}

impl Provider {
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

impl FromStr for Provider {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "local" => Self::Local,
            "openai" => Self::OpenAI,
            "anthropic" => Self::Anthropic,
            "huggingface" => Self::HuggingFace,
            other => Self::Other(other.to_string()),
        })
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub provider: Provider,
    pub id: String,
}

impl FromStr for Model {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once("/") {
            let provider: Provider = a.parse()?;

            return Ok(Self {
                provider,
                id: b.to_string(),
            });
        }

        Err(error::Error::new().with_message("invalid model"))
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.provider, &self.id)
    }
}
