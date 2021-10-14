use std::collections::HashMap;

use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

const IP_ADDR: &str = "127.0.0.1:8000";

mod utils;
use utils::*;

mod responses;

mod examples;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(IP_ADDR).await?;
    println!("Starting HTTP server on: {}", IP_ADDR);

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("[{}] New connection.", addr);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut buf = tokio::io::BufReader::new(reader);
            let mut start_line = String::new();
            let mut header_temp: String = String::new();
            let mut headers = HashMap::new();
            let mut data = Vec::<u8>::new();
            
            loop {
                start_line.clear();
                let n = buf.read_line(&mut start_line).await.unwrap();  // FIXME: Read can fail
                if n == 0 {
                    println!("[{}] Closed connection.", addr);
                    return;
                }
                let start_line = start_line.trim();
                println!("[{}] {}", addr, start_line);

                loop {
                    header_temp.clear();
                    let n = buf.read_line(&mut header_temp).await.unwrap();  // FIXME: Read can fail
                    if n == 0 {
                        // Empty headers - disconnect
                        writer.write_all(responses::BAD_REQUEST).await.unwrap();    // FIXME: unwrap()
                        return;
                    } else if n <= 2 {
                        break;      // Move onto http body reading
                    }
                    
                    if let Some((key, val)) = header_temp.split_once(":") {
                        let key = key.trim();
                        let val = val.trim();

                        headers.insert(key.to_string(), val.to_string());
                    } else {
                        // Invalid header - disconnect
                        writer.write_all(responses::BAD_REQUEST).await.unwrap();    // FIXME: unwrap()
                        return;
                    }
                }

                if !is_keep_alive(&headers) {
                    println!("[{}] DISCONNECTED - keep-alive", addr);
                    return;
                }

                let data_len = get_content_length(&headers);
                if data_len > 0 {
                    data.resize(data_len, 0);
                    let n = buf.read_exact(&mut data).await.unwrap();  // FIXME: Read can fail
                    if n == 0 {
                        // No http body provided when content length was described - disconnect
                        writer.write_all(responses::BAD_REQUEST).await.unwrap();    // FIXME: unwrap()
                        return;
                    }
                }
                
                let (method, path_str, _) = parse_start_line(start_line);
                
                // FIXME: writer.write() can fail
                match method {
                    Method::GET => writer.write_all(examples::static_site::get_request(path_str, &headers, &data).as_slice()).await.unwrap(),
                    _ => writer.write_all(responses::NOT_IMPLEMENTED).await.unwrap(),
                };
            }
        });
    }
}
