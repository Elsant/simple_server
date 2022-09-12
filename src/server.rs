use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received - {}", String::from_utf8_lossy(&mut buffer));

                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(err) => println!("{:?}", err)
                            }
                        },
                        Err(err) => println!("{:?}", err),
                    }
                },
                Err(err) => println!("{:?}", err),
            }
        }
    }
}