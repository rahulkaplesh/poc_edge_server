mod common;
mod edge_server;

fn main() {
    println!("Hello, world!");
    let server = edge_server::EdgeServer::new();
    match server {
        Ok(server) => server.run(),
        Err(err) => println!("Cannot Run edge server: {}", err),
    };
}
