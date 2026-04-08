#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatCompletionTool {
    Function { function: FunctionDefinition },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub strict: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoice {
    Mode(ToolChoiceMode),
    Named(NamedToolChoice),
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {
    Auto,
    None,
    Required,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NamedToolChoice {
    #[serde(rename = "type")]
    pub choice_type: String,
    pub function: NamedFunction,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NamedFunction {
    pub name: String,
}
