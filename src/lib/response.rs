use std::{fmt, collections::HashMap};

pub struct Response {
    pub http_version: String,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: u16, body: Vec<u8>) -> Response {
        let status_text = match status_code {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        }.to_string();

        Response {
            http_version: "HTTP/1.1".to_string(),
            status_code,
            status_text,
            headers: HashMap::new(),
            body,
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn to_http_response(&self) -> Vec<u8> {
        let mut response = format!("{} {} {}\r\n",
                                   self.http_version,
                                   self.status_code,
                                   self.status_text);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        let mut response_bytes = response.into_bytes().to_vec();
        response_bytes.extend_from_slice(&self.body);

        response_bytes
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.to_http_response()).unwrap())
    }
}