mod input;
mod output;

pub use input::*;
pub use output::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Model {
    GPT,
    GPT2,
    GPTJ,
    GPTNeo,
    XLNet,
    Reformer,
    T5,
    Other(String),
}
