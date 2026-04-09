mod provider;
mod registry;

pub use provider::*;
pub use registry::*;

use std::str::FromStr;

use crate::error::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ModelId {
    pub provider: ProviderId,
    pub id: String,
}

impl TryFrom<String> for ModelId {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<ModelId> for String {
    fn from(value: ModelId) -> Self {
        value.to_string()
    }
}

impl FromStr for ModelId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.trim().split_once("/") {
            if a.is_empty() {
                return Err("invalid model provider".into());
            }

            let provider = ProviderId::from(a.to_string());

            return Ok(Self {
                provider,
                id: b.to_string(),
            });
        }

        Err(format!("invalid model '{}'", s).into())
    }
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.provider, &self.id)
    }
}
