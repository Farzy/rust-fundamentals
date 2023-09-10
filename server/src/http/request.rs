use super::method::{Method, MethodError};
use super::{QueryString, QueryStringValue};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abs@sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
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
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // match path.find('?') {
        //     Some(i) => {
        //         // Valid operation because '?' is 1 byte
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     }
        //     None => {  },
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     // Valid operation because '?' is 1 byte
        //     query_string = Some(&path[i+1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?') {
            // Valid operation because '?' is 1 byte
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
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

    for (i, c) in request.char_indices() {
        if c == ' ' || c == '\r' {
            // This code is safe because we know ' ' is 1 byte, so (i+1) skips exactly 1 char
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod(String),
}

impl ParseError {
    fn message(&self) -> String {
        match self {
            Self::InvalidRequest => "Invalid Request".to_string(),
            Self::InvalidEncoding => "Invalid Encoding".to_string(),
            Self::InvalidProtocol => "Invalid Protocol".to_string(),
            Self::InvalidMethod(m) => format!("Invalid Method: {}", m),
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(e: MethodError) -> Self {
        Self::InvalidMethod(e.0)
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
