use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use crate::lib::handler::handle_request;
use crate::lib::request::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        if let Some(request) = Request::from_raw_request(&buffer) {
            let response = handle_request(&request);
            let response_bytes = response.to_http_response();
            stream.write(&response_bytes).unwrap();
            stream.flush().unwrap();
        } else {
            // Send back an error response or handle malformed request
        }
    }
}
