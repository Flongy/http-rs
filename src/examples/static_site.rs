use std::{collections::HashMap, path::Path};

use crate::responses;


pub fn get_request(path_str: &str, _headers: &HashMap<String, String>, _data: &[u8]) -> Vec<u8> {
    let mut path = Path::new("./www/").join(&path_str[1..]);
    if path.is_dir() {
        path = path.join("index.html");
    }

    let mut result = Vec::new();
    
    if path.is_file() {
        #[cfg(debug_assertions)] println!("FOUND! {}", path.to_str().unwrap());
        result.extend(responses::OK);

        // TODO: Add content type detection like this:
        // result.extend(b"Content-Type: text/html\r\n");

        let file_data = std::fs::read(path).unwrap();
        result.extend(format!("Content-Length: {}\r\n\r\n", file_data.len()).as_bytes());   // Write Content-Length and move onto data body
        result.extend(&file_data);
    } else {
        #[cfg(debug_assertions)] println!("NOT FOUND! {}; is_dir: {}", path.to_str().unwrap(), path.is_dir());
        result.extend(responses::NOT_FOUND);
    };
    result
}
