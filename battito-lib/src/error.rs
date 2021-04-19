use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoteParsingError,
    DSLParsingError,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::NoteParsingError
    }
}
