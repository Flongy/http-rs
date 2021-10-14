#![allow(dead_code)]

use std::collections::HashMap;

#[allow(clippy::upper_case_acronyms)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,

    INCORRECT
}

impl Method {
    fn from_str(method: &str) -> Self {
        match method {
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "CONNECT" => Method::CONNECT,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            _ => Method::INCORRECT
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Protocol {
    HTTP_1_1,

    INCORRECT
}

impl Protocol {
    fn from_str(protocol: &str) -> Self {
        match protocol {
            "HTTP/1.1" => Protocol::HTTP_1_1,
            _ => Protocol::INCORRECT
        }
    }
}


pub fn parse_start_line(start_line: &str) -> (Method, &str, Protocol) {
    let mut split = start_line.split_whitespace();

    let method = Method::from_str(split.next().unwrap());
    let path = split.next().unwrap();
    let protocol = Protocol::from_str(split.next().unwrap());

    (method, path, protocol)
}

pub fn get_content_length(headers: &HashMap<String, String>) -> usize {
    if let Some(str_content_length) = headers.get("Content-Length") {
        str_content_length.parse::<usize>().unwrap()
    } else {
        0
    }
}

pub fn is_keep_alive(headers: &HashMap<String, String>) -> bool {
    if let Some(str_connection) = headers.get("Connection") {
        str_connection == "keep-alive"
    } else {
        false
    }
}
