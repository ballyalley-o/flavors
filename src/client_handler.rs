use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct ClientHandler {
    public_path: String
}

impl ClientHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traverssal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for ClientHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> Response {
       match request.method() {
        Method::GET => match request.path() {
            "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
            "/about" => Response::new(StatusCode::Ok, self.read_file("about.html")),
            path => match self.read_file(path) {
                Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                None => Response::new(StatusCode::NotFound, None)
            }
        }
        _ => Response::new(StatusCode::NotFound, None)
       }
    }
}