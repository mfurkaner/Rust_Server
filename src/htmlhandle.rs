#![allow(dead_code)]
use std::fs;
use std::io::Write;
use std::net::TcpStream;

use crate::consts::html;

#[derive(Clone, Copy)]
pub enum RequestType {
    GET,
    POST,
    INVALID
}

pub struct RequestInfo{
    pub command: String,
    pub status_code: String,
    pub html_path: String,
    pub _type: RequestType
}

pub struct Request{
    pub info : RequestInfo,
    pub content: String
}

pub fn validate_request(request: [u8;1024]) -> RequestInfo{
    let mut result: RequestInfo = RequestInfo { 
        command : String::new(),
        status_code: html::INVALID_STATUS_CODE.to_string(),
        html_path: html::INVALID_HTML_PATH.to_string(),
        _type: RequestType::INVALID
    };

    if request.starts_with(html::GET_HTML_CODE.as_bytes()) {
        println!("GET request!");
        result.command = get_request_command(&request);
        result._type = RequestType::GET;
        result.status_code = html::OK_STATUS_CODE.to_string();
    }
    else if request.starts_with(html::POST_HTML_CODE.as_bytes()){
        println!("POST request!");
        result.command = get_request_command(&request);
        result._type = RequestType::POST;
        result.status_code = html::OK_STATUS_CODE.to_string();
        
    }else{
        println!("Invalid request!");
    };

    result
}

fn get_request_command(request: &[u8;1024]) -> String{
    let req_str = String::from_utf8_lossy(&request[..]).to_string();
    let first_line = req_str.lines().next().unwrap();
    let requested_part = first_line.split_whitespace().nth(1).unwrap();

    let mut command : String = String::new();

    if requested_part.starts_with("/") {
        command = requested_part.to_string();
    };

    command
}

fn get_content_len(request: &[u8;1024]) -> usize{
    let req_str = String::from_utf8_lossy(&request[..]).to_string();
    let mut result: usize = 0;
    if req_str.contains("Content-Length"){
        let content__length_line = req_str.lines().find(|&x| x.starts_with("Content-Length:")).unwrap();
        let content_length = content__length_line.split_whitespace().nth(1).unwrap();
        result = content_length.to_string().parse().unwrap();
    }

    return result;
}

fn get_content(request: &[u8;1024]) -> String{

    let content_length = get_content_len(request);

    let req_str = String::from_utf8_lossy(&request[..]).to_string();

    let content = req_str.split_inclusive("\r\n\r\n").nth(1).unwrap();
    if content.len() >= content_length{
        return content.to_string();
    }
    return String::new();
}

pub fn construct_get_request_object(request: &[u8;1024], req_info: &RequestInfo) -> Request{
    return Request{
        info : RequestInfo{
            command : req_info.command.to_string(),
            _type : req_info._type,
            html_path : req_info.html_path.to_string(),
            status_code : req_info.status_code.to_string()
        },
        content : get_content(request)
    };
}

pub fn construct_post_request_object(request: &[u8;1024], req_info: &RequestInfo) -> Request{
    return Request{
        info : RequestInfo { 
            command : req_info.command.to_string(),
            status_code: req_info.status_code.to_string(), 
            html_path: req_info.html_path.to_string(), 
            _type: req_info._type
        },
        content : get_content(request)
    };
}

pub fn construct_invalid_request_object(req_info: &RequestInfo) -> Request{
    return Request{
        info : RequestInfo { 
            command : req_info.status_code.to_string(),
            status_code: req_info.status_code.to_string(), 
            html_path: req_info.html_path.to_string(), 
            _type: req_info._type
        },
        content : String::new()
    };
}

pub fn handle_get_request(request: &mut Request, auth: bool){
    request.info.html_path = if request.info.command == "/" {
        html::LOGIN_HTML_PATH.to_string()
    }else if request.info.command.starts_with("/") && auth{
        html::INDEX_HTML_PATH.to_string()
    }else{
        html::INVALID_HTML_PATH.to_string()
    };


}

pub fn handle_post_request(request: &Request, _auth: bool){
    if request.info.command == "/login" {
        //implement login
    }else{
        //implement login
    };
}

pub fn handle_invalid_request(_request: &Request){
}

pub fn post_html_file(request:&Request, streamInfo: (&TcpStream, &String)){
    let mut stream = streamInfo.0;
    let body = fs::read_to_string(&request.info.html_path).unwrap();

    let response : String;
    if request.info.html_path == html::LOGIN_HTML_PATH {
        let secret_set = format!("secret = 0x{};",streamInfo.1);
        response = construct_html_response_with_insert(&request.info.status_code, &body, &secret_set, "</script>");
    }else{
        response = construct_html_response(&request.info.status_code, &body);
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn post_html_response(body:String, streamInfo: (&TcpStream, &String)){
    let mut stream = streamInfo.0;

    let response = construct_html_response(html::OK_STATUS_CODE, &body);

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


