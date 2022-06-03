use std::io::Read;
use std::net::TcpStream;
use std::time;
use std::time::{Duration, Instant};
use colored::*;

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
    pub authIDs: Vec<(String, time::Instant)>,
    pub IDs_toremove: Vec<String>
}



impl ConnectionHandler {
    pub fn handle_connection(&mut self, mut streamInfo: (TcpStream, String)){
        self.cleanup_auth_ids();

        let mut buffer = [0; 1024];
    
        streamInfo.0.read(&mut buffer).unwrap();
        
        let streamInfoBurrows = (&streamInfo.0, &streamInfo.1);

        let requestInfo = htmlhandle::validate_request(buffer);

        let mut request = match requestInfo._type {
            htmlhandle::RequestType::POST    => htmlhandle::construct_post_request_object(&buffer, &requestInfo),
            htmlhandle::RequestType::GET     => htmlhandle::construct_get_request_object(&buffer, &requestInfo),
            htmlhandle::RequestType::INVALID => htmlhandle::construct_invalid_request_object(&requestInfo),
        };

        let auth = self.check_authentication(&request) || self.check_credentials(&request);
        
        if auth{
            self.authIDs.push( (streamInfoBurrows.1.to_owned(), Instant::now()));
            println!("{}", "The user has auth privileges.".bold().yellow());
        }

        match request.info._type {
            htmlhandle::RequestType::GET     => self.handle_get_request(streamInfoBurrows, &mut request, auth),
            htmlhandle::RequestType::POST    => self.handle_post_request(streamInfoBurrows, &mut request, auth),
            htmlhandle::RequestType::INVALID => self.handle_invalid_request(streamInfoBurrows, &mut request)
        };

        request.content = request.content.trim_matches('\0').to_string();

        if request.content.len() > 0 {
            println!("Content : {}", request.content);
        }

        self.cleanup_valid_ids();

    }

    fn send_response(&mut self, response: String, streamInfo: (&TcpStream, &String)){
        htmlhandle::post_html_response(response, streamInfo);
    }

    fn handle_get_request(&mut self, streamInfo: (&TcpStream, &String), request: &mut htmlhandle::Request, auth: bool){
        htmlhandle::handle_get_request(request, auth);
        htmlhandle::post_html_file(&request, streamInfo);
    }

    fn handle_post_request(&mut self, streamInfo: (&TcpStream, &String), request: &mut htmlhandle::Request, mut _auth: bool){

        // if the requester is auth or , give auth priv to the new id
        if _auth{
            self.send_response(streamInfo.1.to_string(), streamInfo);
        }else{
            self.send_response("FAIL".to_string(), streamInfo);
        };

        htmlhandle::handle_post_request(request, _auth);
    }

    fn handle_invalid_request(&mut self, streamInfo: (&TcpStream, &String), request: &mut htmlhandle::Request){
        htmlhandle::handle_invalid_request(request);
        htmlhandle::post_html_file(&request, streamInfo);
    }

    fn get_credentials(&self, post_request_content: String) -> Credentials {
        
        let map = jsonhandler::map_from_json(post_request_content);
        
        return Credentials{
            conn_id     : if map.is_empty() {"0".to_string()} else {map.get("conn_id").unwrap().to_string()},  
            hashid_sec  : if map.is_empty() {"0".to_string()} else {map.get("hashid_sec").unwrap().to_string()}, 
            hashpw_sec  : if map.is_empty() {"0".to_string()} else {map.get("hashpw_sec").unwrap().to_string()}, 
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

            if self.authIDs[i].0 == command_split[0]{

                let expected_hash = hash::hash_str(html::ID_HASH.to_owned() + &html::PW_HASH.to_owned() + &self.authIDs[i].0.to_owned());
                let expected_hash = format!("{:x}",expected_hash);

                if  expected_hash == command_split[1] {
                    self.authIDs.remove(i);
                    break true;
                }
            }
            i += 1;
        };
    }

    fn check_credentials(&mut self, request: &Request) -> bool {
        let credentials = self.get_credentials(request.content.clone());
        self.IDs_toremove.push(credentials.conn_id.to_string());

        for i in 0..self.validIDs.len(){
            if credentials.conn_id != self.validIDs[i].as_str() {
                if i == self.validIDs.len() - 1 {println!("Unknown connection id : {} valid ids are : {:?}",credentials.conn_id, self.validIDs)};
                continue;
            };
            let expected_id = html::ID_HASH;
            let expected_pw = html::PW_HASH;
            let expected_id_hash = format!("{:x}",hash::hash_str( expected_id.to_owned() + &credentials.conn_id ) );
            let expected_pw_hash = format!("{:x}",hash::hash_str( expected_pw.to_owned() + &credentials.conn_id ) );

            if expected_id_hash == credentials.hashid_sec{
                if expected_pw_hash == credentials.hashpw_sec {
                    println!("{}", "Login success!".green().bold());
                    return true;
                }else{
                    println!("{}","User entered incorrect password.".red().bold());
                };
            }else{
                println!("{}","User entered incorrect credentials.".red().bold());
            }
            return false;
        }
        return false;
    }

    fn is_auth(&self, str : &str)->bool{
        for a in self.authIDs.to_owned(){
            if a.0 == str{
                return true;
            }
        }
        return false;
    }

    fn cleanup_auth_ids(&mut self){
        let mut i: usize = 0;
        while i < self.authIDs.len(){
            if  self.authIDs[i].1.elapsed() > Duration::new(180, 0){
                println!("{} {}",format!("{}",self.authIDs[i].0).yellow().bold(),"lost the auth privilage due to timeout.".red());
                self.authIDs.remove(i);
            }else{
                i += 1;
            }
        }
    }

    fn cleanup_valid_ids(&mut self){
        // ensure we are not closing auth connections to get new connections
        if self.validIDs.len() == connection::MAX_ALLOWED_CONNECTIONS {
            for i in 0..self.validIDs.len(){
                if !self.is_auth(&self.validIDs[i] ){
                    self.validIDs.remove(i);
                    break;
                }
            }
        };

        // remove the dead connections
        while !self.IDs_toremove.is_empty() {
            for i in 0.. self.validIDs.len() {
                if self.validIDs[i] == self.IDs_toremove.last().unwrap().to_owned(){
                    self.validIDs.remove(i);
                    break;
                }
            } 

            for i in 0.. self.authIDs.len(){
                if self.authIDs[i].0 == self.IDs_toremove.last().unwrap().to_owned(){
                    self.authIDs.remove(i);
                    break;
                }
            }

            self.IDs_toremove.pop();
        }
    }

    pub fn print_auth_ids(&self){
        print!("[");
        for a in 0..self.authIDs.len(){
            let rem = Duration::new(180, 0).as_secs().saturating_sub(self.authIDs[a].1.elapsed().as_secs()) ;
            print!("{}{}{}{}{}{}","{".blue(), "\"", format!("{}", self.authIDs[a].0 ).yellow(),  "\", remaining : ", format!("{}s",rem).red(), "}".blue() );
            if a + 1 != self.authIDs.len(){
                print!("{}"," , ".blue());
            }
        } 
        print!("]\n");
    }

}