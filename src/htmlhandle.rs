#![allow(dead_code)]
use std::fs;
use std::io::Write;
use std::net::TcpStream;

use colored::Colorize;

use crate::connection::Credentials;
use crate::consts::html;
use crate::hash;
use crate::connection;

#[derive(Clone, Copy, PartialEq)]
pub enum RequestType {
    GET,
    POST,
    INVALID
}

#[derive(Clone, Copy, PartialEq)]
pub enum HtmlGetAction{
    LoginPage,
    Logout,
    Index,
    SubPage,
    InvalidPage
}

pub struct RequestInfo{
    pub command: String,
    pub status_code: String,
    pub html_path: String,
    pub _type: RequestType,
    pub credentials: connection::Credentials
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
        _type: RequestType::INVALID,
        credentials : Credentials{
            hashid_sec : String::new(),
            hashpw_sec : String::new(),
            conn_id : String::new()
        }
    };

    if request.starts_with(html::GET_HTML_CODE.as_bytes()) {
        result.command = get_request_command(&request);
        result._type = RequestType::GET;
        result.status_code = html::OK_STATUS_CODE.to_string();
    }
    else if request.starts_with(html::POST_HTML_CODE.as_bytes()){
        result.command = get_request_command(&request);
        result._type = RequestType::POST;
        result.status_code = html::OK_STATUS_CODE.to_string();
        
    }else{
        println!("{}","Invalid request!".red().bold());
    };

    result
}

pub fn construct_request_object(request: &[u8;1024]) -> Request{
    let requestInfo = validate_request(*request);

    let request_obj = match requestInfo._type {
        RequestType::POST    => construct_post_request_object(&request, &requestInfo),
        RequestType::GET     => construct_get_request_object(&request, &requestInfo),
        RequestType::INVALID => construct_invalid_request_object(&requestInfo),
    };

    println!("Request details : {} {}", match request_obj.info._type{
            RequestType::POST    => "POST".bold(),
            RequestType::GET     => "GET".bold(),
            RequestType::INVALID => "".bold()
        }, request_obj.info.command.bold()
    );
    if request_obj.info._type == RequestType::POST {
        println!("Request contents : {}", request_obj.content.yellow().dimmed());
    }
    
    return request_obj;
}

pub fn handle_get_request(request: &mut Request, auth: bool) -> HtmlGetAction{
    let action = get_http_action_from_command(request);
    request.info.html_path = match action {
        HtmlGetAction::InvalidPage => html::INVALID_HTML_PATH.to_string(),
        HtmlGetAction::LoginPage => html::LOGIN_HTML_PATH.to_string(),
        HtmlGetAction::Logout => html::LOGIN_HTML_PATH.to_string(),
        HtmlGetAction::Index =>     if auth {
                                        html::INDEX_HTML_PATH.to_string()
                                    } else {
                                        html::INVALID_HTML_PATH.to_string()},
        HtmlGetAction::SubPage =>   if auth {
                                        "html/".to_string() + request.info.command.split("/").last().unwrap() + ".html" 
                                    } else {
                                        html::INVALID_HTML_PATH.to_string()
                                    }
    };

    println!("Sending '{}' to the client.", request.info.html_path.bright_yellow());

    action
}

pub fn post_html_file(request:&Request, streamInfo: (&TcpStream, &String)){
    let mut stream = streamInfo.0;
    let body = match fs::read_to_string(&request.info.html_path){
        Ok(a) => a,
        Err(_) => String::new(),
    };

    let response : String;

    if request.info.html_path == html::LOGIN_HTML_PATH {
        let secret_set = format!("hash1 = 0x{};",streamInfo.1);
        response = construct_html_response_with_insert(&request.info.status_code, &body, &secret_set, "</script>");
    }else if request.info._type == RequestType::GET{
        let _hash = hash::hash_str(html::ID_HASH.to_owned() + html::PW_HASH + streamInfo.1);
        let secret_set = format!("hash1 = 0x{};hash2 = 0x{:x};",streamInfo.1, _hash);
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

fn get_http_action_from_command(request: &mut Request) -> HtmlGetAction{
    let get_command = request.info.command.clone();
    let mut result = HtmlGetAction::InvalidPage;
    let split = get_command.split("/");
    if get_command == "/" {
        result = HtmlGetAction::LoginPage;
    }else if get_command.starts_with("/") {
        if split.to_owned().count() >= 4 {
            result = HtmlGetAction::SubPage;
        }else if split.to_owned().count() == 3 && split.to_owned().last().unwrap() == "logout" {
            // if the last part of the command is logout, the last segment is the connection id. We need it to remove from auth_id list when logging off
            request.info.credentials.conn_id = split.to_owned().nth(1).unwrap().chars().filter(|x| x.is_alphanumeric()).collect();
            result = HtmlGetAction::Logout;
        }
        else{
            result = HtmlGetAction::Index;
        }
    }
    return result;
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

fn construct_get_request_object(request: &[u8;1024], req_info: &RequestInfo) -> Request{
    return Request{
        info : RequestInfo{
            command : req_info.command.to_string(),
            _type : req_info._type,
            html_path : req_info.html_path.to_string(),
            status_code : req_info.status_code.to_string(),
            credentials : req_info.credentials.clone()
        },
        content : get_content(request)
    };
}

fn construct_post_request_object(request: &[u8;1024], req_info: &RequestInfo) -> Request{
    return Request{
        info : RequestInfo { 
            command : req_info.command.to_string(),
            status_code: req_info.status_code.to_string(), 
            html_path: req_info.html_path.to_string(), 
            _type: req_info._type,
            credentials : req_info.credentials.clone()
        },
        content : get_content(request)
    };
}

fn construct_invalid_request_object(req_info: &RequestInfo) -> Request{
    return Request{
        info : RequestInfo { 
            command : req_info.status_code.to_string(),
            status_code: req_info.status_code.to_string(), 
            html_path: req_info.html_path.to_string(), 
            _type: req_info._type,
            credentials : req_info.credentials.clone()
        },
        content : String::new()
    };
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


