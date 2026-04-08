mod parse;

pub use parse::*;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Http(reqwest::Error),
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(v) => write!(f, "{}", v),
            Self::Http(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for Error {}
