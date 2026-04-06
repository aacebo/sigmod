#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
pub enum ScorerType {
    Llm,
    Classifier,
    Levenshtein,
    Semantic,
    Code,
}

impl std::fmt::Display for ScorerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Llm => write!(f, "llm"),
            Self::Classifier => write!(f, "classifier"),
            Self::Levenshtein => write!(f, "levenshtein"),
            Self::Semantic => write!(f, "semantic"),
            Self::Code => write!(f, "code"),
        }
    }
}
