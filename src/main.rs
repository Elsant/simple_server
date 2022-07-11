use http::Request;
use http::Method;
use server::Server;

mod server;
mod http;

fn main() {
    let string = String::from("127.0.0.1:3000");
    let server = Server::new(string);
    server.run();
}
