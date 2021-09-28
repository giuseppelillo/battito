use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Error {
    EventParsingError,
    DSLParsingError(ParsingError),
    UnexpectedError,
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    Generic,
    EuclideanError(EuclideanError),
}

#[derive(Debug, PartialEq)]
pub enum EuclideanError {
    NGreaterThanM,
    RGreaterEqualThanM,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::EventParsingError
    }
}
