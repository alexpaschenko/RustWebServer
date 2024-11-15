use std::net::TcpListener;
use std::io::Read;
use crate::http::request::{Request, RequestParseError};
use crate::http::response::{Response, Status};

pub struct Server {
    address: String,
}

pub trait RequestHandler {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_parse_error(&self, e: &RequestParseError) -> Response {
        println!("Bad request: {}", e);
        Response::new(Status::BadRequest, None)
    }
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self, request_handler: &impl RequestHandler) {
        println!("Server listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, client_addr)) => {
                    println!("Client {} connected", client_addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => request_handler.handle_request(&request),
                                Err(e) => request_handler.handle_parse_error(&e)
                            };
                            if let Err(e) = response.write(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => println!("Error reading from stream: {}", e),
                    }
                },
                Err(e) => println!("Client connection error: {}", e)
            }
        }
    }
}