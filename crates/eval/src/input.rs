use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub enum Input {
    Classifier(classifier::Input),
    Judge(judge::Input),
}
