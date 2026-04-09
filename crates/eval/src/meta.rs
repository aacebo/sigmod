use std::collections::BTreeMap;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Meta(BTreeMap<String, serde_json::Value>);

impl Meta {
    pub const REQUEST_ID: &str = "request_id";
    pub const USAGE: &str = "usage";
    pub const ELAPSED_MS: &str = "elapsed_ms";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: impl AsRef<str>) -> Option<T> {
        serde_json::from_value(self.0.get(key.as_ref())?.clone()).ok()
    }

    pub fn with(
        mut self,
        key: impl Into<String>,
        value: impl serde::Serialize,
    ) -> Result<Self, serde_json::Error> {
        self.0.insert(key.into(), serde_json::to_value(value)?);
        Ok(self)
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        value: impl serde::Serialize,
    ) -> Result<Option<serde_json::Value>, serde_json::Error> {
        Ok(self.0.insert(key.into(), serde_json::to_value(value)?))
    }

    pub fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Usage {
    pub tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
}
