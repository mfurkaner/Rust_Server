use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use crate::consts::html;

pub fn handle_connection(streamInfo: (TcpStream, u64)){
    let mut buffer = [0; 1024];

    let mut stream = streamInfo.0;

    stream.read(&mut buffer).unwrap();

    println!("A request is received from {}", stream.peer_addr().unwrap());

    let (status_code, html_path) = validate_request(buffer, html::EXPECTED_BASIC_REQUEST);

    if status_code == html::POST_HTML_CODE{
        handle_post_request(stream);
    }
    else{
        post_html_file(html_path, status_code, stream);
    };


}

fn validate_request(buffer: [u8;1024], expected: &[u8; 16]) -> (&str, &str){
    if buffer.starts_with(expected) {
        println!("Valid request!");
        return (html::INDEX_STATUS_CODE, html::INDEX_HTML_PATH);
    }
    else if buffer.starts_with(html::POST_HTML_CODE.as_bytes()){
        println!("Post request!");
        return (html::POST_HTML_CODE, html::INDEX_HTML_PATH);
    };
    println!("Invalid request!");
    println!("{}", String::from_utf8_lossy(&buffer[..]));
    return (html::INVALID_STATUS_CODE, html::INVALID_HTML_PATH);
}

fn handle_post_request(mut stream:TcpStream) -> String{
    /* 
    let request = String::from_utf8_lossy(&buffer).to_string();
    let mut flag = false;
    let mut content_len = 0;

    request.split("{}")

    for word in request.split_whitespace(){
        if flag {
            content_len = word.parse::<i32>().unwrap();
        }
        if word.find("Content-Length: ") != None{
            flag = true;
        }
    }*/

    let mut content = [0; 1024];

    stream.read(&mut content).unwrap();
    println!("post request content: {}", String::from_utf8_lossy(&content[..]));

    return String::new();
}

fn post_html_file(html_path: &str, status_code: &str, mut stream: TcpStream){
    let index_html = fs::read_to_string(html_path).unwrap();
    let header = format!("Content-Length: {}",index_html.len());

    let response = format!("{}\r\n{}\r\n\r\n{}", status_code, header, index_html);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}