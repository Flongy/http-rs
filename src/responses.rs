#![allow(dead_code)]

pub const OK: &[u8] = b"HTTP/1.1 200 OK\r\n";
pub const BAD_REQUEST: &[u8] = b"HTTP/1.1 400 Bad Request\r\n";
pub const NOT_FOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n";
pub const NOT_IMPLEMENTED: &[u8] = b"HTTP/1.1 501 Not Implemented\r\n";
