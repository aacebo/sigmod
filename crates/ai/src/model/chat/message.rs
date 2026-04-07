use super::tool::ChatCompletionToolCall;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatCompletionMessage {
    System {
        content: Content,
        name: Option<String>,
    },
    Developer {
        content: Content,
        name: Option<String>,
    },
    User {
        content: Content,
        name: Option<String>,
    },
    Assistant {
        content: Option<String>,
        refusal: Option<String>,
        annotations: Option<Vec<Annotation>>,
        tool_calls: Option<Vec<ChatCompletionToolCall>>,
        name: Option<String>,
    },
    Tool {
        content: Content,
        tool_call_id: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    UrlCitation { url_citation: UrlCitation },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UrlCitation {
    pub start_index: u32,
    pub end_index: u32,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
    File { file: FileContent },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageUrl {
    pub url: String,
    pub detail: Option<ImageDetail>,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    Auto,
    Low,
    High,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileContent {
    pub file_data: Option<String>,
    pub file_id: Option<String>,
    pub filename: Option<String>,
}
