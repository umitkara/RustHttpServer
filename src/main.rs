mod lib;

use lib::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:7878".to_string());
    server.run();
}
