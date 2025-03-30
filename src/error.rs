use std::fmt::Display;

/// An error while parsing a relative human time duration string.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// There was an invalid unit.
    InvalidUnit,
    /// There was an invalid number.
    InvalidNumber,
    /// The input was empty while parsing the duration.
    EmptyDurationInput,
    /// There was an invalid relative type.
    InvalidRelativeType,
    /// The input was empty while parsing the relative type.
    EmptyRelativeInput,
    /// There was multiple relative types.
    MultipleRelativeTypes,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUnit => write!(f, "invalid unit"),
            Self::InvalidNumber => write!(f, "invalid number"),
            Self::EmptyDurationInput => write!(f, "empty duration input"),
            Self::InvalidRelativeType => write!(f, "invalid relative type"),
            Self::EmptyRelativeInput => write!(f, "empty relative input"),
            Self::MultipleRelativeTypes => write!(f, "multiple relative types"),
        }
    }
}
