use std::{time::{SystemTime, Duration}, collections::HashMap};

pub enum ResponseCodes {
    OK,
    NotFound,
    Created,
}

impl ResponseCodes {
    pub fn as_response_header(&self) -> String{
        let pre = String::from("HTTP/1.1 ");
        pre + match self {
            ResponseCodes::OK => "200 OK\r\n",
            ResponseCodes::NotFound => "404 Not found\r\n",
            ResponseCodes::Created => "201 Created\r\n"
        }
    }
}

pub enum ContentType {
    HTML,
    JSON,
    TEXT
}

impl ContentType {
    pub fn to_string(&self) -> String{
        match self {
            Self::HTML => "text/html",
            Self::JSON => "application/json",
            Self::TEXT => "text/plain"
        }.to_string() 
    }
}

pub enum Headers {
    ContentType(ContentType),
    CORS(String),
}

impl Headers {
    pub fn to_string(&self) -> String{
        match self {
            Headers::ContentType(content) => {
                ("Content-Type: ".to_string() + &content.to_string() + "\n\r").to_string()
            }
            Headers::CORS(origins) => {
                ("Access-Control-Allow-Origin: ".to_string() + origins + "\n\r").to_string()
            }
        }
    }
}

enum ResponseParts {
    Status,
    Headers,
    Whitespace,
    Content
}

pub struct Response {

    content: HashMap<ResponseParts, String>
}

impl Response {
    pub fn new() -> Self {
        Self { content: HashMap::new() }
    }
}
