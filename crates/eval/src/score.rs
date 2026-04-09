use crate::{Context, Evaluate, classifier, judge};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScorerInput {
    Classifier(classifier::Input),
    Judge(judge::Input),
}

#[async_trait::async_trait]
impl Evaluate for ScorerInput {
    type Output = ScorerOutput;

    async fn evaluate(&self, ctx: &mut Context) -> Result<Self::Output, error::Error> {
        Ok(match self {
            Self::Classifier(v) => v.evaluate(ctx).await?.into(),
            Self::Judge(v) => v.evaluate(ctx).await?.into(),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScorerOutput {
    Classifier(classifier::Output),
    Judge(judge::Output),
    Error(error::Error),
}

impl From<classifier::Output> for ScorerOutput {
    fn from(value: classifier::Output) -> Self {
        Self::Classifier(value)
    }
}

impl From<judge::Output> for ScorerOutput {
    fn from(value: judge::Output) -> Self {
        Self::Judge(value)
    }
}

impl From<error::Error> for ScorerOutput {
    fn from(value: error::Error) -> Self {
        Self::Error(value)
    }
}
