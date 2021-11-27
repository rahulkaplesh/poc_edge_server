use std::collections::HashMap;
use std::net::SocketAddr;
use std::io::{self};

use super::common::Message;

use message_io::network::{NetEvent, Transport, Endpoint};
use message_io::node::{self, NodeHandler, NodeListener};
use bincode;

struct EdgeClientInfo {
    addr: SocketAddr,
    endpoint: Endpoint,
}

pub struct EdgeServer {
    handler: NodeHandler<()>,
    node_listener: Option<NodeListener<()>>,
    clients: HashMap<String, EdgeClientInfo>,
}

impl EdgeServer {
    pub fn new(serverString: &str) -> io::Result<EdgeServer> {
        let (handler, node_listener) = node::split::<()>();

        let listen_addr = serverString;
        handler.network().listen(Transport::FramedTcp, listen_addr)?;

        println!("Discovery server running at {}", listen_addr);

        Ok(EdgeServer {
            handler,
            node_listener: Some(node_listener),
            clients: HashMap::new()
        })
    }

    pub fn run(mut self) {
        let node_listener = self.node_listener.take().unwrap();
        node_listener.for_each(move |event| match event.network() {
            NetEvent::Connected(_, _) => unreachable!(),
            NetEvent::Accepted(_, _) => (),    // All endpoints are being accepted This can be used for filtering denied endpoints
            NetEvent::Message(endpoint, data_recieved) => {
                let message: Message = bincode::deserialize(&data_recieved).unwrap();
                match message {
                    Message::RegisterClient(name, address) => {
                        self.register(&name, address, endpoint);
                    },
                    Message::UnregisterClient(name) => {
                        self.unregister(&name);
                    },
                    Message::DataTransferMessage(name, message) => {
                        self.process_message(&name, &message);
                    },
                }
            },
            NetEvent::Disconnected(endpoint) =>  {
                let client = self.clients.iter().find(|(_, info)| info.endpoint == endpoint);
                if let Some(client) = client {
                    let name = client.0.to_string();
                    self.unregister(&name);
                }
            },
        });
    }

    fn register(&mut self, name: &str, address: SocketAddr, endpoint: Endpoint) {
        if !self.clients.contains_key(name) {
            self.clients.insert(name.to_string(), EdgeClientInfo{ addr: address, endpoint });
            println!("Added client '{}' with ip {}", name, address);
        }
        else {
            println!("Client with name '{}' already exists, regiter with another name!", name);
        }
    }

    fn unregister(&mut self, name: &str) {
        if let Some(info) = self.clients.remove(name) {
            println!("Removed Client '{}' with ip {}", name, info.addr);
        }
        else {
            println!("Cannot Unregister a non-existing client '{}'", name);
        }
    }

    fn process_message(&mut self, name: &str, message: &str) {
        println!("Message from client '{}' recieved is : {}", name, message);
    }
}