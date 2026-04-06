mod group;

pub use group::*;

use std::{backtrace::Backtrace, collections::BTreeMap, sync::Arc};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    message: Option<String>,
    fields: BTreeMap<String, String>,

    #[serde(skip)]
    backtrace: Option<Arc<Backtrace>>,

    #[serde(skip)]
    inner: Option<Arc<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Error {
    pub fn new() -> Self {
        Self {
            message: None,
            fields: BTreeMap::new(),
            backtrace: None,
            inner: None,
        }
    }

    pub fn message(&self) -> Option<&str> {
        match &self.message {
            None => None,
            Some(v) => Some(v.as_str()),
        }
    }

    pub fn field(&self, name: &str) -> Option<&str> {
        match &self.fields.get(name) {
            None => None,
            Some(v) => Some(v),
        }
    }

    pub fn backtrace(&self) -> Option<&Backtrace> {
        match &self.backtrace {
            None => None,
            Some(v) => Some(v),
        }
    }

    pub fn inner(&self) -> Option<&dyn std::error::Error> {
        match &self.inner {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }

    pub fn with_message(mut self, message: impl std::fmt::Display) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn with_field(mut self, name: &str, value: impl std::fmt::Display) -> Self {
        self.fields.insert(name.to_string(), value.to_string());
        self
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for Error {
    fn from(value: T) -> Self {
        Self {
            message: None,
            fields: BTreeMap::new(),
            backtrace: None,
            inner: Some(Arc::new(value)),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[ERROR]")?;

        if let Some(backtrace) = &self.backtrace {
            writeln!(f, "\tbacktrace: {}", backtrace)?;
        }

        if let Some(error) = &self.inner {
            writeln!(f, "\tinner error: {}", error)?;
        }

        if let Some(message) = &self.message {
            writeln!(f, "\tmessage: {}", message)?;
        }

        Ok(())
    }
}
