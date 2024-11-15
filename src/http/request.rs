use crate::http::next_token;
use crate::http::query_string::QueryString;
use crate::http::request::RequestParseError::{InvalidRequest, UnsupportedProtocol};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::{FromStr, Utf8Error};

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    DELETE,
    POST,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "POST" => Ok(Method::POST),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(MethodParseError::new(s.into())),
        }
    }
}

pub struct MethodParseError {
    method: String
}

impl MethodParseError {
    fn new(method: String) -> Self {
        Self {
            method
        }
    }
}

pub enum RequestParseError {
    InvalidRequest,
    InvalidEncoding,
    UnsupportedMethod(String),
    UnsupportedProtocol(String)
}

impl RequestParseError {
    fn message(&self) -> String {
        match self {
            RequestParseError::InvalidRequest => "Invalid Request".to_string(),
            RequestParseError::InvalidEncoding => "Invalid Encoding".to_string(),
            RequestParseError::UnsupportedMethod(method) => format!("Unsupported Method {}", method),
            RequestParseError::UnsupportedProtocol(protocol) => format!("Unsupported Protocol {}", protocol),
        }
    }
}

impl Error for RequestParseError {}

impl Display for RequestParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for RequestParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for RequestParseError {
    fn from(_value: Utf8Error) -> Self {
        RequestParseError::InvalidEncoding
    }
}

impl From<MethodParseError> for RequestParseError {
    fn from(value: MethodParseError) -> Self {
        RequestParseError::UnsupportedMethod(value.method)
    }
}

#[derive(Debug)]
pub struct Request<'request_buffer> {
    method: Method,
    path: &'request_buffer str,
    query_string: Option<QueryString<'request_buffer>>,
}

impl <'request_buffer> Request<'request_buffer> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &'request_buffer str {
        self.path
    }

    pub fn query_string(&self) -> Option<&QueryString<'request_buffer>> {
        self.query_string.as_ref()
    }
}

impl<'request_buffer> TryFrom<&'request_buffer[u8]> for Request<'request_buffer> {
    type Error = RequestParseError;

    fn try_from(buf: &'request_buffer[u8]) -> Result<Request<'request_buffer>, Self::Error> {
        let request_str = std::str::from_utf8(buf)?;
        let (method, request_str) = next_token(request_str, ' ').ok_or(InvalidRequest)?;
        let (mut path, request_str) = next_token(request_str, ' ').ok_or(InvalidRequest)?;
        let (protocol, _) = next_token(request_str, '\r').ok_or(InvalidRequest)?;
        let mut query_string = None;
        if let Some(tup) = next_token(path, '?') {
            path = tup.0;
            query_string = Some(QueryString::from(tup.1));
        };

        if protocol != "HTTP/1.1" {
            return Err(UnsupportedProtocol(protocol.into()));
        }

        let method: Method = method.parse()?;

        Ok(Request {
            method,
            path: path.into(),
            query_string,
        })
    }
}