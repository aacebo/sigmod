mod criterion;
mod input;
mod output;

pub use criterion::*;
pub use input::*;
pub use output::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Model {
    Gpt,
    Gpt2,
    GptJ,
    GptNeo,
    XlNet,
    Reformer,
    T5,
    #[serde(untagged)]
    Other(String),
}
