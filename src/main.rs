fn main() {
    let string = String::from("127.0.0.1:3000");
    let server = Server::new(string);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}