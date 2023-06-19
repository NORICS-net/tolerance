use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, TryFromIntError};

#[derive(Debug, PartialEq)]
pub enum ToleranceError {
    ParseError(String),
    Overflow(String),
}

impl std::error::Error for ToleranceError {}

impl From<ParseFloatError> for ToleranceError {
    fn from(pfe: ParseFloatError) -> Self {
        ToleranceError::ParseError(match pfe.to_string().as_str() {
            "invalid float literal" => "invalid allowance literal".to_string(),
            "cannot parse float from empty string" => {
                "cannot parse allowance from empty string".to_string()
            }
            err => "Unknown error: ".to_string() + err,
        })
    }
}

impl From<TryFromIntError> for ToleranceError {
    fn from(t: TryFromIntError) -> Self {
        ToleranceError::Overflow(t.to_string())
    }
}

impl Display for ToleranceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ToleranceError::*;
        let text = match self {
            ParseError(text) | Overflow(text) => text.as_str(),
        };
        write!(f, "{text}")
    }
}
