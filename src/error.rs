use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, TryFromIntError};

#[derive(Debug, Eq, PartialEq)]
pub enum ToleranceError {
    ParseError(String),
    Overflow(String),
    ParseEmptyStr(&'static str),
    ValidationError(String),
}

impl std::error::Error for ToleranceError {}

impl From<ParseFloatError> for ToleranceError {
    fn from(pfe: ParseFloatError) -> Self {
        Self::ParseError(pfe.to_string())
    }
}

impl From<TryFromIntError> for ToleranceError {
    fn from(t: TryFromIntError) -> Self {
        Self::Overflow(t.to_string())
    }
}

/// The error that never happens.
impl From<Infallible> for ToleranceError {
    fn from(_: Infallible) -> Self {
        Self::ParseError(String::new())
    }
}

impl Display for ToleranceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ToleranceError::*;
        let text = match self {
            ParseError(text) | Overflow(text) => text.as_str(),
            ParseEmptyStr(type_r) => &format!("Cannot parse an empty string into {type_r}."),
            ValidationError(text) => text.as_str(),
        };
        write!(f, "{text}")
    }
}

impl ToleranceError {
    /// Helper-method to create a `ParseError`.
    #[inline]
    pub fn parse_err<R>(text: impl Into<String>) -> Result<R, Self> {
        Err(Self::ParseError(text.into()))
    }
}
