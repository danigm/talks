mod server;

use self::server::HttpServer;

fn main() {
    let server = HttpServer::new();
    println!("Running server {server}");
    server.run();
}
