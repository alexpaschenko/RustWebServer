use std::fmt::{Display, Formatter, Result};
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl Status {
    pub fn message(&self) -> &str {
        match self {
            Status::OK => "OK",
            Status::BadRequest => "Bad request",
            Status::NotFound => "Not found",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", *self as u16)
    }
}

pub struct Response {
    status: Status,
    body: Option<String>,
}

impl Response {
    pub fn new(status: Status, body: Option<String>) -> Self {
        Self { status, body }
    }

    pub fn write(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let body = match &self.body {
            None => "",
            Some(body) => body,
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status, self.status.message(), body)
    }
}