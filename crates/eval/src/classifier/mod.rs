mod category;
mod input;
mod label;
mod output;

pub use category::*;
pub use input::*;
pub use label::*;
pub use output::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Model {
    Bart,
    Bert,
    Deberta,
    DebertaV2,
    DistilBert,
    Roberta,
    /// custom url.
    Other(String),
}
