use std::net::{TcpListener, TcpStream};

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
                Ok((stream, addr)) => {

                }
                Err(e) => println!("failed to establish a connection: {}", e),
            }
        }
    }
}
