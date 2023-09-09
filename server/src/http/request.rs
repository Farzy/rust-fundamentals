use super::Method;
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

    // GET /search?name=abs@sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // };

        // match str::from_utf8(buf).or(Err(ParseError::InvalidRequest)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }

        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidRequest))?;

        let request = str::from_utf8(buf)?;
        todo!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {},
    //         None => break,
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            // This code is safe because we know ' ' is 1 byte, so (i+1) skips exactly 1 char
            return Some((&request[..i], &request[i+1..]))
        }
    }
    todo!()
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
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

impl Error for ParseError {}
