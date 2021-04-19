use nannou_osc::CommunicationError;
use std::io;

#[derive(Debug)]
pub enum Error {
    InputError,
    UDPError,
    ParsingError,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::InputError
    }
}

impl From<CommunicationError> for Error {
    fn from(_: CommunicationError) -> Self {
        Error::UDPError
    }
}

impl From<battito_lib::error::Error> for Error {
    fn from(_: battito_lib::error::Error) -> Self {
        Error::ParsingError
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::InputError
    }
}
