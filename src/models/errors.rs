use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum Error {
    StringToU32(std::num::ParseIntError),
    NumTryFromIntError(std::num::TryFromIntError),
    Db(sqlx::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::StringToU32(err) => write!(f, "std::num::ParseIntError: {}", err),
            Error::NumTryFromIntError(err) => write!(f, "std::num::TryFromIntError: {}", err),
            Error::Db(err) => write!(f, "sqlx::Error: {}", err),
        }
    }
}
