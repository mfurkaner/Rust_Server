#![allow(non_snake_case)]

use std::net::TcpListener;

mod connection;
mod consts;
mod hash;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.accept().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let random = format!("{:x}",rand::random::<u32>());
        println!("Connection id is : {}", random);

        let hash = hash::hash_str(random);
        println!("Hash of the connection is : {:x}", hash);
        
        let streamInfo = (stream, hash);

        connection::handle_connection(streamInfo);
    }
}