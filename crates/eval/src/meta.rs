use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Meta {
    pub request_id: Option<String>,
    pub elapse_time: Option<u64>,
    pub usage: Option<Usage>,

    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Usage {
    pub tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
}
