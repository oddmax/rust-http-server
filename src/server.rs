use std::io::Read;
use std::net::{TcpListener};
use std::convert::TryFrom;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {

    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening to {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(_) => {}
                                Err(e) => println!("failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("failed to read from the connection: {}", e),
                    }
                }
                Err(e) => println!("failed to establish a connection: {}", e),
            }
        }
    }
}
