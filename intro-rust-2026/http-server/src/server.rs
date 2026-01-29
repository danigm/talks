use std::io::prelude::*;
use std::string::FromUtf8Error;
use std::fmt::Display;
use std::net::{TcpListener, TcpStream};

pub enum Error {
    IOError(std::io::Error),
    U8Error(FromUtf8Error),
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::U8Error(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}

pub struct HttpServer {
    port: i32,
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(port: Option<i32>) -> Option<HttpServer> {
        let default_port = port.unwrap_or(8080);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", default_port));
        match listener {
            Ok(listener) => Some(HttpServer { port: default_port, listener }),
            Err(error) => {
                eprintln!("Can't bind to {}", default_port);
                None
            }
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        // accept connections and process them serially
        for stream in self.listener.incoming() {
            self.handle_client(&mut stream?);
        }

        Ok(())
    }

    fn look_for_path(&self, content: &String) -> Option<String> {
        // looking for GET to get the requested path
        // This assumes that the GET and path is in the buffer completely and not splitted
        match content.find("GET") {
            Some(idx) => {
                let space = content[idx+4..].find(" ").unwrap();
                Some(String::from(&content[idx+5..idx+4+space]))
            },
            None => None
        }
    }

    fn handle_client(&self, stream: &mut TcpStream) -> Result<(), Error> {
        println!("Client connected");
        let mut buf = [0u8; 256];
        let mut r = stream.read(&mut buf)?;
        let mut path = None;
        while r >= 256 {
            let content = String::from_utf8(buf[0..r].to_vec())?;

            if let Some(p) = self.look_for_path(&content) {
                path = Some(p);
            }
            print!("{}", content);
            r = stream.read(&mut buf)?;
        }

        let content = String::from_utf8(buf[0..r].to_vec())?;
        if let Some(p) = self.look_for_path(&content) {
            path = Some(p);
        }

        println!("{}", content);

        let p = match path {
            Some(s) if s == "" => String::from("index.html"),
            Some(s) => s,
            None => String::from("index.html"),
        };

        // TODO: check for existence for ret and open the file to send with smaller chunks to the
        // stream to avoid loading the whole file in memory.
        let (ret, content) = match std::fs::read_to_string(&p) {
            Ok(s) => (format!("200 OK"), s),
            Err(_) => (format!("404 Not Found"), format!("404 {p} not found"))
        };

        let mut response = format!("HTTP/1.1 {ret}");
        response += "\r\n\r\n";
        response += &format!("{content}");
        response += "\r\n\r\n";

        stream.write(response.as_bytes());

        Ok(())
    }
}

impl Display for HttpServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom Server: http://localhost:{}", self.port)
    }
}
