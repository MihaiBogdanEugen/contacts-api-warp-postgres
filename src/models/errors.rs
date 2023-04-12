use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    DbError(sqlx::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::ParseError(err) => write!(f, "Cannot parse parameter: {}", err),
            Error::DbError(err) => write!(f, "DB error: {}", err),
        }
    }
}

impl Reject for Error {}
