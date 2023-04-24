use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::string::FromUtf8Error;

use base64::DecodeError;

#[derive(Debug)]
pub enum Error {
    /// Pagination info (page_no or page_size) cannot be converted from String to u32
    StringToU32(String),

    /// Database specific error, wrapper on top of a sqlx::Error
    Db(String),

    /// Entity with provided ID is not found in the repository
    NotFound { id: i32 },

    /// The HTTP Authorization header value is invalid
    InvalidAuthHeader,

    /// The HTTP Authorization header contains a valid value but the scheme is other than `Basic`
    InvalidScheme(String),

    /// The value expected as a base64 encoded `String` is not encoded correctly
    InvalidBase64Value(String),

    /// The provided binary is not a valid UTF-8 character
    InvalidUtf8Value(String),

    /// The external validation api call failed
    ReqwestMiddleware(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::StringToU32(message) => write!(
                f,
                "Pagination info (page_no or page_size) cannot be converted from String to u32: {}",
                message
            ),
            Error::Db(scheme) => write!(f, "Database specific error: {}", scheme),
            Error::NotFound { id } => write!(
                f,
                "Entity with provided ID ({}) is not found in the repository.",
                id
            ),
            Error::InvalidAuthHeader => write!(
                f,
                "Invalid value provided for the HTTP Authorization header"
            ),
            Error::InvalidScheme(scheme) => {
                write!(f, "The scheme provided ({}) is not Basic", scheme)
            }
            Error::InvalidBase64Value(message) => {
                write!(f, "The value have an invalid base64 encoding: {}", message)
            }
            Error::InvalidUtf8Value(message) => write!(f, "Invalid UTF-8 Provided: {}", message),
            Error::ReqwestMiddleware(message) => {
                write!(f, "The external validation api call failed {}", message)
            }
        }
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Error::InvalidBase64Value(err.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::InvalidUtf8Value(err.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Db(err.to_string())
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(err: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestMiddleware(err.to_string())
    }
}
