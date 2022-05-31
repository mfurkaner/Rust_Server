#![allow(non_snake_case)]

use std::net::TcpListener;

mod connection;
mod consts;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.accept().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        connection::handle_connection(stream);
    }
}