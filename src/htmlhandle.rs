#![allow(dead_code)]
use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use crate::consts::html;

pub fn validate_request(request: [u8;1024], expected: &[u8; 16]) -> (&str, &str){
    if request.starts_with(expected) {
        println!("Valid request!");
        return (html::INDEX_STATUS_CODE, html::LOGIN_HTML_PATH);
    }
    else if request.starts_with(html::POST_HTML_CODE.as_bytes()){
        println!("Post request!");
        return (html::POST_HTML_CODE, html::LOGIN_HTML_PATH);
    };
    println!("Invalid request!");
    println!("{}", String::from_utf8_lossy(&request[..]));
    return (html::INVALID_STATUS_CODE, html::INVALID_HTML_PATH);
}

pub fn handle_post_request(mut streamInfo: (&TcpStream, &String)) -> String{
    let mut content = [0; 1024];

    streamInfo.0.read(&mut content).unwrap();
    let result = String::from_utf8_lossy(&content[..]).to_string();
    println!("post request content: {}", result);

    return result;
}

pub fn post_html_file(html_path: &str, status_code: &str, streamInfo: (&TcpStream, &String)){
    let mut stream = streamInfo.0;
    let body = fs::read_to_string(html_path).unwrap();

    let secret_set = format!("secret = 0x{};",streamInfo.1);
    let response = construct_html_response_with_insert(status_code, &body, &secret_set, "</script>");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn construct_html_response(status_code: &str, body: &str) -> String{
    let header = format!("Content-Length: {}",body.len());
    format!("{}\r\n{}\r\n\r\n{}", status_code, header, body)
}

fn construct_html_response_with_insert(status_code: &str, body: &str, to_add: &str, add_before: &str) -> String{
    let header = format!("Content-Length: {}",body.len() + to_add.len());
    let mut response = format!("{}\r\n{}\r\n\r\n{}", status_code, header, body);
    response.insert_str(response.find(add_before).unwrap(), to_add);
    response
}


