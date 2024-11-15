use std::fs;
use crate::http::request::{Method, Request};
use crate::http::response::{Response, Status};
use crate::server::RequestHandler;

pub struct WebsiteHandler {
    root_path: String,
}

impl WebsiteHandler {
    pub fn new(root_path: String) -> Self {
        Self { root_path }
    }

    fn read_file(&self, path: &str) -> Option<String> {
        fs::read_to_string(format!("{}/{}", self.root_path, path)).ok()
    }
}

impl RequestHandler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        dbg!("Request: {}", request);
        match request.method() {
            Method::GET => {
                match request.path() {
                    "/" => Response::new(Status::OK, self.read_file("index.html")),
                    "/hello" =>Response::new(Status::OK,  {
                        match request.query_string() {
                            None => Some("Add a query string to see it printed".to_string()),
                            Some(query_string) => Some(format!("{:?}", query_string))
                        }
                    }),
                    path => match self.read_file(path) {
                        Some(data) => Response::new(Status::OK, Some(data)),
                        None => Response::new(Status::NotFound, None),
                    }
                }
            },
            _ => Response::new(Status::NotFound, None)
        }
    }
}