use std::collections::BTreeMap;

use crate::judge::Model;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Input {
    /// the model to use.
    pub model: Model,

    /// the judges name.
    #[validate(min_length = 2)]
    pub name: String,

    /// the input text being evaluated.
    #[validate(min_length = 3)]
    pub text: String,

    #[validate(min_length = 3)]
    pub prompt: String,

    /// options set to the LLM when applicable.
    #[serde(default)]
    pub options: Option<BTreeMap<String, serde_json::Value>>,
}
