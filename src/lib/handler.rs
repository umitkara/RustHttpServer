use crate::lib::request::Request;
use crate::lib::response::Response;
use std::{io::Read, fs};

pub fn handle_request(request: &Request) -> Response {
    match request.method.as_ref() {
        "GET" => handle_get_request(&request),
        "POST" => handle_post_request(&request),
        _ => Response::new(500, Vec::new()),
    }
}

fn handle_post_request(request: &Request) -> Response {
    // TODO: Think a smart way to handle POST requests dynamically
    let mut response = Response::new(200, Vec::new());
    response
}

fn handle_get_request(request: &Request) -> Response {
    let mut response = Response::new(200, Vec::new());

    let file_path = if request.path == "/" {
        format!("{}{}", "./public", "/index.html")
    } else {
        format!("{}{}", "./public", request.path)
    };
    let file_contents = read_file(&file_path);

    match file_contents {
        Some(contents) => {
            response.body = contents;
            response.set_header("Content-Type", &get_mime_type(&file_path));
        }
        None => {
            response.status_code = 404;
        }
    }

    response
}

pub fn read_file(filename: &str) -> Option<Vec<u8>> {
    match fs::File::open(filename) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            Some(contents)
        }
        Err(_) => None,
    }
}

pub fn get_mime_type(file_path: &str) -> String {
    let file_extension = file_path.split('.').last().unwrap_or("");
    // This could be improved by using a HashMap or Enum
    let mime_type = match file_extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    };

    mime_type.to_string()
}