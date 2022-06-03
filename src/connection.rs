use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use crate::consts::html;
use crate::consts::connection;
use crate::htmlhandle;
use crate::htmlhandle::Request;
use crate::jsonhandler;
use crate::hash;

#[derive(Clone, PartialEq)]
struct Credentials{
    pub conn_id: String,
    pub hashid_sec: String,
    pub hashpw_sec: String
}

pub struct ConnectionHandler{
    pub validIDs: Vec<String>,
    pub authIDs: Vec<String>,
    pub IDs_toremove: Vec<String>
}


impl ConnectionHandler {
    pub fn handle_connection(&mut self, mut streamInfo: (TcpStream, String)){
        let mut buffer = [0; 1024];
    
        streamInfo.0.read(&mut buffer).unwrap();
        
        let streamInfoBurrows = (&streamInfo.0, &streamInfo.1);
    
        println!("A request is received from {}", streamInfo.0.peer_addr().unwrap());

        let requestInfo = htmlhandle::validate_request(buffer);

        let mut request = match requestInfo._type {
            htmlhandle::RequestType::POST    => htmlhandle::construct_post_request_object(&buffer, &requestInfo),
            htmlhandle::RequestType::GET     => htmlhandle::construct_get_request_object(&buffer, &requestInfo),
            htmlhandle::RequestType::INVALID => htmlhandle::construct_invalid_request_object(&requestInfo),
        };

        let mut i = 0;
        let auth = self.check_authentication(&request);

        match request.info._type {
            htmlhandle::RequestType::GET     => self.handle_get_request(streamInfoBurrows, &mut request, auth),
            htmlhandle::RequestType::POST    => self.handle_post_request(streamInfoBurrows, &mut request, auth),
            htmlhandle::RequestType::INVALID => self.handle_invalid_request(streamInfoBurrows, &mut request)
        };
        
        if request.content.len() > 0 {
            println!("Content : {}", request.content);
        }

        self.cleanup_valid_ids();

    }

    fn send_response(&mut self, response: String, streamInfo: (&TcpStream, &String)){
        htmlhandle::post_html_response(response, streamInfo);
    }

    fn handle_get_request(&mut self, streamInfo: (&TcpStream, &String), request: &mut htmlhandle::Request, mut auth: bool){
        htmlhandle::handle_get_request(request, auth);
        htmlhandle::post_html_file(&request, streamInfo);
    }

    fn handle_post_request(&mut self, streamInfo: (&TcpStream, &String), request: &mut htmlhandle::Request, mut auth: bool){

        let credentials = self.get_credentials(request.content.clone());
        self.IDs_toremove.push(credentials.conn_id.to_string());

        if self.check_credentials(credentials.to_owned()){
            self.authIDs.push(streamInfo.1.to_string());
            auth = true;
            self.send_response(streamInfo.1.to_string(), streamInfo);
        }else{
            self.send_response("FAIL".to_string(), streamInfo);
            auth = false;
        };

        htmlhandle::handle_post_request(request, auth);
    }

    fn handle_invalid_request(&mut self, streamInfo: (&TcpStream, &String), request: &mut htmlhandle::Request){
        htmlhandle::handle_invalid_request(request);
        htmlhandle::post_html_file(&request, streamInfo);
    }

    fn get_credentials(&self, post_request_content: String) -> Credentials {
        
        let map = jsonhandler::map_from_json(post_request_content);
        
        return Credentials{
            conn_id     : map.get("conn_id").unwrap().to_string(), 
            hashid_sec  : map.get("hashid_sec").unwrap().to_string(), 
            hashpw_sec  : map.get("hashpw_sec").unwrap().to_string() 
        };
    }

    fn check_authentication(&mut self, request: &Request) -> bool {
        let mut i = 0;
        let mut command = request.info.command.to_owned();
        if command.starts_with("/") { command.remove(0);}
        let command_split : Vec<&str>= command.split("/").collect();

        return loop {
            if i >= self.authIDs.len() || command_split.len() < 2{ 
                break false
            ;}
            
            if self.authIDs[i] == command_split[0]{

                let expected_hash = hash::hash_str(html::ID_HASH.to_owned() + &html::PW_HASH.to_owned() + &self.authIDs[i].to_owned());
                let expected_hash = format!("{:x}",expected_hash);

                if  expected_hash == command_split[1] {
                    break true;
                }
            }
            i += 1;
        };
    }

    fn check_credentials(&mut self, credentials: Credentials) -> bool {
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
                    return true;
                }else{
                    println!("Did you forget your password?");
                };
            }else{
                println!("Wrong credentials.");
            }
        }
        return false;
    }

    fn cleanup_valid_ids(&mut self){
        if self.validIDs.len() == connection::MAX_ALLOWED_CONNECTIONS {
            self.validIDs.remove(0);
        };
        'outer: for i in 0..self.validIDs.len(){
            for j in 0.. self.IDs_toremove.len() {
                if self.validIDs[i] == self.IDs_toremove[j]{
                    self.validIDs.remove(i);
                    self.IDs_toremove.remove(j);
                    break 'outer;
                }
            } 
        }
    }


}