#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: JsonSchema },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JsonSchema {
    pub name: String,
    pub schema: serde_json::Value,
    pub strict: Option<bool>,
}
