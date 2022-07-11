use http::request::Request;
use server::Server;

fn main() {
    let string = String::from("127.0.0.1:3000");
    let server = Server::new(string);
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr)
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }

    pub mod method {
        pub enum Method {
            GET(String),
            POST,
            DELETE(u64)
        }
    }
}