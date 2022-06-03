#![allow(unused_variables)]
#![allow(dead_code)]
// Common constants

    // HTML constants
pub mod html{

    pub const EXPECTED_BASIC_REQUEST: &[u8; 16] = b"GET / HTTP/1.1\r\n";

    pub const INDEX_HTML_PATH: &str = "html/index.html";
    pub const INDEX_STATUS_CODE: &str = "HTTP/1.1 200 OK";

    pub const POST_HTML_CODE: &str = "POST";

    pub const LOGIN_HTML_PATH: &str = "html/login.html";
        
    pub const INVALID_HTML_PATH: &str = "html/404.html";
    pub const INVALID_STATUS_CODE: &str = "HTTP/1.1 404 NOT FOUND";

    pub const ID_HASH: &str = "74b2f64";
    pub const PW_HASH: &str = "c6a240";
}


pub mod connection{

    pub const LOCAL_IP_ADDR: &str   = "192.168.1.35:7878";
    pub const PUBLIC_IP_ADDR: &str  = "212.156.207.204:7878";

    pub const MAX_ALLOWED_CONNECTIONS: usize = 5;

}


pub mod hash{
    pub const FIRST:u32 = 37;
    pub const A:u32 = 54059;
    pub const B:u32 = 76963;
}