mod server;

use self::server::HttpServer;

fn main() {
    if let Some(server) = HttpServer::new(Some(8080)) {
        println!("Running server {server}");
        server.run();
    }
}
