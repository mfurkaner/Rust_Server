#![allow(non_snake_case)]

use std::net::TcpListener;


mod connection;
mod consts;
mod hash;
mod htmlhandle;
mod jsonhandler;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.accept().unwrap();

    let mut conn_handle = connection::ConnectionHandler{
        validIDs : Vec::new()
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let random = format!("{:x}",rand::random::<u32>());
        println!("Connection id is : {}", random);
        
        if conn_handle.validIDs.len() > consts::connection::MAX_ALLOWED_CONNECTIONS {
            conn_handle.validIDs.remove(0);
        };
        conn_handle.validIDs.push(random.to_owned());

        let streamInfo = (stream, random);

        conn_handle.handle_connection(streamInfo);
    }
}