use openssl::error::Error as OpensslError;
use openssl::error::ErrorStack as OpensslErrorStack;
use openssl::ssl::Error as SslError;
use serde_json::Error as SerdeJsonError;
use serde_urlencoded::ser::Error as SerdeUrlEncodeSerError;
use std::io::Error as IOError;
use std::result::Result;
use thiserror::Error;
use ureq::Error as UreqError;

#[derive(Error, Debug)]
pub enum AlipayError {
    #[error("IoError: {0}")]
    IOError(String),
    #[error("OpensslError: {0}")]
    OpensslError(String),
    #[error("OpensslErrorStack: {0}")]
    OpensslErrorStack(String),
    #[error("SslError: {0}")]
    SslError(String),
    #[error("UreqError: {0}")]
    UreqError(String),
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(String),
    #[error("SerdeUrlEncodeSerError: {0}")]
    SerdeUrlEncodeSerError(String),
}

impl From<IOError> for AlipayError {
    fn from(error: IOError) -> Self {
        AlipayError::IOError(error.to_string())
    }
}

impl From<OpensslError> for AlipayError {
    fn from(error: OpensslError) -> Self {
        AlipayError::OpensslError(error.to_string())
    }
}
impl From<OpensslErrorStack> for AlipayError {
    fn from(error: OpensslErrorStack) -> Self {
        AlipayError::OpensslErrorStack(error.to_string())
    }
}
impl From<SslError> for AlipayError {
    fn from(error: SslError) -> Self {
        AlipayError::SslError(error.to_string())
    }
}
impl From<UreqError> for AlipayError {
    fn from(error: UreqError) -> Self {
        AlipayError::UreqError(error.to_string())
    }
}
impl From<SerdeJsonError> for AlipayError {
    fn from(error: SerdeJsonError) -> Self {
        AlipayError::SerdeJsonError(error.to_string())
    }
}
impl From<SerdeUrlEncodeSerError> for AlipayError {
    fn from(error: SerdeUrlEncodeSerError) -> Self {
        AlipayError::SerdeUrlEncodeSerError(error.to_string())
    }
}

pub type AlipayResult<T> = Result<T, AlipayError>;
