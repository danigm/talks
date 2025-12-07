use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: &mut TcpStream) {
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

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}
