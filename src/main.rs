mod common;
mod edge_server;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Edge Server")
        .version("1.0")
        .author("Rahul Kaplesh <rahulkaplesh@gmail.com")
        .about("an edge server")
        .arg( 
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("String")
                .help("Takes an address")
                .takes_value(true),
        ).get_matches();
    let server;
    if let Some(server_string) = matches.value_of("address") {
        server = edge_server::EdgeServer::new(server_string);
        match server {
            Ok(server) => server.run(),
            Err(err) => println!("Cannot Run edge server: {}", err),
        };
    } else {
        println!("Please pass -a param");
    }
    
}
