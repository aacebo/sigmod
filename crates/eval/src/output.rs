use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Output {
    Classifier(classifier::Output),
    Judge(judge::Output),
}
