use nannou_osc::CommunicationError;
use std::io;
use actix_web::error;
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
#[display(fmt = "Error!")]
pub enum ServiceError {
    InputError,
    UDPError,
    ParsingError,
}

impl From<io::Error> for ServiceError {
    fn from(_: io::Error) -> Self {
        ServiceError::InputError
    }
}

impl From<CommunicationError> for ServiceError {
    fn from(_: CommunicationError) -> Self {
        ServiceError::UDPError
    }
}

impl From<battito_lib::error::Error> for ServiceError {
    fn from(_: battito_lib::error::Error) -> Self {
        ServiceError::ParsingError
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(_: serde_json::Error) -> Self {
        ServiceError::InputError
    }
}

impl error::ResponseError for ServiceError {}
