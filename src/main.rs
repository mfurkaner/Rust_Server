#![allow(non_snake_case)]

use std::net::TcpListener;

use colored::Colorize;


mod connection;
mod consts;
mod hash;
mod htmlhandle;
mod jsonhandler;

fn main() {
    let listener = TcpListener::bind(consts::connection::LOCAL_IP_ADDR).unwrap();
    println!("\n{} {}\n", "Server is configured. Listening on :".green().bold(), consts::connection::PUBLIC_IP_ADDR.bright_blue().bold());

    let mut conn_handle = connection::ConnectionHandler{
        IDs_toremove : Vec::new(),
        validIDs : Vec::new(),
        authIDs : Vec::new()
    };

    listener.accept().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let random = format!("{:x}",rand::random::<u32>());
        println!("{} {} {} {}\n","A request is received. Connection id :".green(), random.blue().bold(), "Public IP :".green() , stream.peer_addr().expect("Peer IP get failed.").to_string().yellow());
        
        conn_handle.validIDs.push(random.to_owned());

        let streamInfo = (stream, random);

        conn_handle.handle_connection(streamInfo);

        if conn_handle.validIDs.len() > 0 {
            println!("Valid ids : {}", format!("{:?}",conn_handle.validIDs).blue());
        }
        if conn_handle.authIDs.len() > 0 {
            print!("Auth ids : ");
            conn_handle.print_auth_ids();
        }
        println!("{}","-----------------------------------------------------------------------------------".bold());

    }
}