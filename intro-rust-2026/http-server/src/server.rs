use std::io::prelude::*;
use std::fmt::Display;
use std::net::{TcpListener, TcpStream};

pub struct HttpServer {
    port: i32,
    listener: TcpListener,
}

impl HttpServer {
    pub fn new() -> HttpServer {
        let default_port = 8080;
        let listener = TcpListener::bind(format!("127.0.0.1:{}", default_port));
        // Unwrap because Result
        // TODO: Handle the error correctly
        let listener = listener.unwrap();
        HttpServer {
            port: default_port,
            listener,
        }
    }

    pub fn run(&self) {
        // accept connections and process them serially
        for stream in self.listener.incoming() {
            self.handle_client(&mut stream.unwrap());
        }
    }

    fn handle_client(&self, stream: &mut TcpStream) {
        println!("Client connected");
        let mut buf = [0u8; 256];
        let mut r = stream.read(&mut buf).unwrap();
        while r >= 256 {
            print!("{}", String::from_utf8(buf[0..r].to_vec()).unwrap());
            r = stream.read(&mut buf).unwrap();
        }
        println!("{}", String::from_utf8(buf[0..r].to_vec()).unwrap());

        stream.write(String::from("HTTP/1.1 200 OK\r\n\r\nHello world!\r\n").as_bytes());
        stream.write(String::from("\r\n").as_bytes());
    }
}

impl Display for HttpServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom Server: http://localhost:{}", self.port)
    }
}
