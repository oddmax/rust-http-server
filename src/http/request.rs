use super::method::{Method, MethodError};
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

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;


        let mut query_str = None;
        /* other ways to parse query string
        match path.find('?') {
            None => {}
            Some(index) => {
                query_str = Some(&path[index + 1..]);
                path = &path[..index]
            }
        }

        let q = path.find('?');
        if q.is_some() {
            let i = q.unwrap();
            query_str = Some(&path[index + 1..]);
            path = &path[..index];
        }
        */

        if let Some(index) = path.find('?') {
            query_str = Some(&path[index + 1..]);
            path = &path[..index];
        }

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    return None;
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self { Self::InvalidMethodError }
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