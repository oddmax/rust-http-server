use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /* Alternatives to error handling
            match str::from_utf8(buf) {
                Ok(request) => {}
                Err(_) => return Err(ParseError::InvalidEncoding),
            }
            match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
                Ok(request) => {}
                Err(e) => return Err(e),
            };
        */

        let request = str::from_utf8(buf)?;

        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethodError
}

impl Error for ParseError {

}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethodError => "Invalid Method Error",
        }
    }
}