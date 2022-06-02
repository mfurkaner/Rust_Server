// Common constants

    // HTML constants
pub mod html{

    pub const EXPECTED_BASIC_REQUEST: &[u8; 16] = b"GET / HTTP/1.1\r\n";

    pub const INDEX_HTML_PATH: &str = "html/login.html";
    pub const INDEX_STATUS_CODE: &str = "HTTP/1.1 200 OK";

    pub const POST_HTML_CODE: &str = "POST";
        
    pub const INVALID_HTML_PATH: &str = "html/404.html";
    pub const INVALID_STATUS_CODE: &str = "HTTP/1.1 404 NOT FOUND";
}


pub mod hash{
    pub const FIRST:u32 = 37;
    pub const A:u32 = 54059;
    pub const B:u32 = 76963;
}