mod common;
mod discovery_server;

fn main() {
    println!("Hello, world!");
    let server = discovery_server::EdgeServer::new();
    match server {
        Ok(server) => server.run(),
        Err(err) => println!("Cannot Run edge server: {}", err),
    };
}
