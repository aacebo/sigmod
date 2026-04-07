use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Meta {
    pub request_id: Option<String>,
    pub elapse_time: Option<u64>,

    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
