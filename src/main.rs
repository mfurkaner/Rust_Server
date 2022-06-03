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
    println!("Server is configured. Listening on : {}", consts::connection::PUBLIC_IP_ADDR);

    let mut conn_handle = connection::ConnectionHandler{
        IDs_toremove : Vec::new(),
        validIDs : Vec::new(),
        authIDs : Vec::new()
    };

    listener.accept().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let random = format!("{:x}",rand::random::<u32>());
        println!("{} {} {} {}","A request is received. Connection id :".green(), random.blue(), "Public IP :".green() , stream.peer_addr().expect("Peer IP get failed.").to_string().yellow());
        
        conn_handle.validIDs.push(random.to_owned());

        let streamInfo = (stream, random);

        conn_handle.handle_connection(streamInfo);

        println!("Connection ended. \nValid ids : {:?}\nAuth ids : {:?}", conn_handle.validIDs, conn_handle.authIDs);

        println!("\n");
    }
}