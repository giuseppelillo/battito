use nannou_osc::CommunicationError;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    NoteParsingError,
    DSLParsingError,
    InputError,
    UDPError,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::DSLParsingError
    }
}

impl From<CommunicationError> for Error {
    fn from(_: CommunicationError) -> Self {
        Error::UDPError
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::DSLParsingError
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::NoteParsingError
    }
}
