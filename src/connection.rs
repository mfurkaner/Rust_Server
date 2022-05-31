use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

const INDEX_HTML_PATH: &str = "html/index.html";
const INDEX_STATUS_CODE: &str = "HTTP/1.1 200 OK";

const INVALID_HTML_PATH: &str = "html/404.html";
const INVALID_STATUS_CODE: &str = "HTTP/1.1 404 NOT FOUND";

pub fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request is received : ");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let (status_code, html_path) = validate_request(buffer, b"GET / HTTP/1.1\r\n");

    post_html_file(html_path, status_code, stream);
}

fn validate_request(buffer: [u8;1024], expected: &[u8; 16]) -> (&str, &str){
    if buffer.starts_with(expected) {
        println!("VALID REQUEST!");
        return (INDEX_STATUS_CODE, INDEX_HTML_PATH);
    };
    println!("Invalid request!");
    return (INVALID_STATUS_CODE, INVALID_HTML_PATH);
}

fn post_html_file(html_path: &str, status_code: &str, mut stream: TcpStream){
    let index_html = fs::read_to_string(html_path).unwrap();
    let header = format!("Content-Length: {}",index_html.len());

    let response = format!("{}\r\n{}\r\n\r\n{}", status_code, header, index_html);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}