pub mod classifier;
mod decision;
pub mod judge;
pub mod math;
mod meta;

pub use ai::model::{ModelId, ProviderId};
pub use decision::*;
pub use meta::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Scorer {
    Classifier(classifier::Scorer),
    Judge(judge::Scorer),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScorerOutput {
    Classifier(classifier::Output),
    Judge(judge::Output),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct EvalRequest {
    /// request identifier.
    #[serde(default)]
    pub request_id: Option<String>,

    /// the input text being evaluated.
    #[validate(min_length = 3)]
    pub text: String,

    /// the scorers to use.
    #[validate(min_items = 1)]
    pub scorers: Vec<Scorer>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvalResult {
    /// metadata
    #[serde(rename = "$meta")]
    pub meta: Meta,

    /// the score.
    pub score: f32,

    /// the final decision.
    pub decision: Decision,

    /// the individual scorer results
    pub scorers: Vec<ScorerOutput>,
}

#[async_trait::async_trait]
pub trait Evaluate {
    type Output;

    async fn evaluate(
        &self,
        text: &str,
        client: &ai::client::Client,
    ) -> Result<Self::Output, error::Error>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EvalId(ulid::Ulid);

impl EvalId {
    const PREFIX: &str = "eval_";
}

impl std::str::FromStr for EvalId {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s
            .strip_prefix(Self::PREFIX)
            .ok_or_else(|| error::Error::new().with_message("invalid eval identifier"))?;

        let ulid = raw.parse::<ulid::Ulid>().map_err(error::Error::from)?;
        Ok(EvalId(ulid))
    }
}

impl std::fmt::Display for EvalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl serde::Serialize for EvalId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // alloc-free-ish: stack buffer via format is fine here
        let s = format!("{}{}", Self::PREFIX, self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for EvalId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        let raw = s
            .strip_prefix(Self::PREFIX)
            .ok_or_else(|| serde::de::Error::custom("invalid eval identifier"))?;

        let ulid = raw
            .parse::<ulid::Ulid>()
            .map_err(serde::de::Error::custom)?;

        Ok(EvalId(ulid))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::EvalId;

    const ULID: &str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
    const EVAL_ID: &str = "eval_01ARZ3NDEKTSV4RRFFQ69G5FAV";

    #[test]
    fn display_includes_eval_prefix() {
        let eval_id = EvalId::from_str(EVAL_ID).unwrap();

        assert_eq!(eval_id.to_string(), EVAL_ID);
    }

    #[test]
    fn from_str_accepts_prefixed_ulid() {
        let eval_id = EvalId::from_str(EVAL_ID);

        assert!(eval_id.is_ok());
    }

    #[test]
    fn from_str_rejects_missing_prefix() {
        let eval_id = EvalId::from_str(ULID);

        assert!(eval_id.is_err());
    }

    #[test]
    fn from_str_rejects_invalid_ulid_payload() {
        let eval_id = EvalId::from_str("eval_not-a-ulid");

        assert!(eval_id.is_err());
    }

    #[test]
    fn serde_serializes_as_prefixed_string() {
        let eval_id = EvalId::from_str(EVAL_ID).unwrap();
        let serialized = serde_json::to_string(&eval_id).unwrap();

        assert_eq!(serialized, format!("\"{EVAL_ID}\""));
    }

    #[test]
    fn serde_deserializes_from_prefixed_string() {
        let eval_id: EvalId = serde_json::from_str(&format!("\"{EVAL_ID}\"")).unwrap();

        assert_eq!(eval_id.to_string(), EVAL_ID);
    }

    #[test]
    fn serde_rejects_missing_prefix() {
        let eval_id = serde_json::from_str::<EvalId>(&format!("\"{ULID}\""));

        assert!(eval_id.is_err());
    }

    #[test]
    fn serde_round_trip_preserves_value() {
        let eval_id = EvalId::from_str(EVAL_ID).unwrap();
        let serialized = serde_json::to_string(&eval_id).unwrap();
        let deserialized: EvalId = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, eval_id);
    }
}
