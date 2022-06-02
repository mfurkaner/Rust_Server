use std::io::Read;
use std::net::TcpStream;

use crate::consts::html;
use crate::htmlhandle;
use crate::jsonhandler;
use crate::hash;

struct Credentials{
    pub conn_id: String,
    pub hashid_sec: String,
    pub hashpw_sec: String
}

pub struct ConnectionHandler{
    pub validIDs: Vec<String>
}


impl ConnectionHandler {
    pub fn handle_connection(&mut self, mut streamInfo: (TcpStream, String)){
        let mut buffer = [0; 1024];
    
        streamInfo.0.read(&mut buffer).unwrap();
        
        let streamInfoBurrows = (&streamInfo.0, &streamInfo.1);
    
        println!("A request is received from {}", streamInfo.0.peer_addr().unwrap());
    
        let (status_code, html_path) = htmlhandle::validate_request(buffer, html::EXPECTED_BASIC_REQUEST);
    
        if status_code == html::POST_HTML_CODE{
            let request_content = htmlhandle::handle_post_request(streamInfoBurrows);
            let credentials = Self::get_credentials(request_content);
            Self::check_credentials(self, credentials);
        };
    
        htmlhandle::post_html_file(html_path, status_code, streamInfoBurrows);
    }

    fn get_credentials(post_request_content: String) -> Credentials {
        
        let map = jsonhandler::map_from_json(post_request_content);
        
        return Credentials{
            conn_id     : map.get("conn_id").unwrap().to_string(), 
            hashid_sec  : map.get("hashid_sec").unwrap().to_string(), 
            hashpw_sec  : map.get("hashpw_sec").unwrap().to_string() 
        };
    }

    fn check_credentials(&mut self, credentials: Credentials){
        for i in 0..self.validIDs.len(){
            if credentials.conn_id != self.validIDs[i].as_str() {
                if i == self.validIDs.len() - 1 {println!("Unknown connection id : {} valid ids are : {:?}",credentials.conn_id, self.validIDs)};
                continue;
            };
            let expected_id = format!("{:x}", hash::hash_str("ali".to_string()));
            let expected_pw = format!("{:x}", hash::hash_str("atabak".to_string()));
            let expected_id_hash = format!("{:x}",hash::hash_str( expected_id + &credentials.conn_id ) );
            let expected_pw_hash = format!("{:x}",hash::hash_str( expected_pw + &credentials.conn_id ) );

            println!("expected id: {} - got: {}",expected_id_hash, credentials.hashid_sec);
            println!("expected pw: {} - got: {}",expected_pw_hash, credentials.hashpw_sec);

            if expected_id_hash == credentials.hashid_sec{
                if expected_pw_hash == credentials.hashpw_sec {
                    println!("Welcome back!");
                }else{
                    println!("Did you forget your password?");
                };
            }else{
                println!("Wrong credentials.");
            }
            self.validIDs.remove(i);
            break;
        }
    }
}