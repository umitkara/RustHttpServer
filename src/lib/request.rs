use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: String::new(),
            path: String::new(),
            http_version: String::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn from_raw_request(raw_request: &[u8]) -> Option<Self> {
        let request = String::from_utf8_lossy(raw_request);
        let mut lines = request.lines();

        let mut headers = HashMap::new();
        let mut body = Vec::new();
        let mut method = String::new();
        let mut path = String::new();
        let mut http_version = String::new();

        if let Some(request_line) = lines.next() {
            let request_line_parts: Vec<&str> = request_line.split_whitespace().collect();
            // Check if the request line has 3 parts for the method, path and HTTP version
            if request_line_parts.len() == 3 {
                method = request_line_parts[0].to_string();
                path = request_line_parts[1].to_string();
                http_version = request_line_parts[2].to_string();
            } else {
                return None;
            }
        }

        let mut body_lines = false;
        for line in lines {
            if line.is_empty() {
                body_lines = true;
                continue;
            }

            if body_lines {
                body.extend_from_slice(line.as_bytes());
            } else {
                let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
                if header_parts.len() == 2 {
                    headers.insert(header_parts[0].to_string(), header_parts[1].to_string());
                }
            }
        }

        Some(Request {
            method,
            path,
            http_version,
            headers,
            body,
        })
    }
}