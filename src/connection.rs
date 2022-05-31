use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use crate::consts::html;

pub fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("A request is received from {}", stream.peer_addr().unwrap());

    let (status_code, html_path) = validate_request(buffer, html::EXPECTED_BASIC_REQUEST);

    post_html_file(html_path, status_code, stream);
}

fn validate_request(buffer: [u8;1024], expected: &[u8; 16]) -> (&str, &str){
    if buffer.starts_with(expected) {
        println!("Valid request!");
        return (html::INDEX_STATUS_CODE, html::INDEX_HTML_PATH);
    };
    println!("Invalid request!");
    return (html::INVALID_STATUS_CODE, html::INVALID_HTML_PATH);
}

fn post_html_file(html_path: &str, status_code: &str, mut stream: TcpStream){
    let index_html = fs::read_to_string(html_path).unwrap();
    let header = format!("Content-Length: {}",index_html.len());

    let response = format!("{}\r\n{}\r\n\r\n{}", status_code, header, index_html);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}