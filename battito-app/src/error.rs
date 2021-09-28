use nannou_osc::CommunicationError;
use std::io;

#[derive(Debug)]
pub enum BattitoError {
    InputError,
    UDPError,
    ParsingError,
    OSCPacketError,
}

impl From<io::Error> for BattitoError {
    fn from(_: io::Error) -> Self {
        BattitoError::InputError
    }
}

impl From<CommunicationError> for BattitoError {
    fn from(_: CommunicationError) -> Self {
        BattitoError::UDPError
    }
}

impl From<battito_lib::pattern::error::Error> for BattitoError {
    fn from(_: battito_lib::pattern::error::Error) -> Self {
        BattitoError::ParsingError
    }
}

impl From<serde_json::Error> for BattitoError {
    fn from(_: serde_json::Error) -> Self {
        BattitoError::InputError
    }
}
