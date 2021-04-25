use hook_protocol::hook_client;
use hook_protocol::{hook_prot, hook_prot::HookProtocol};

use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

mod read_file;

fn listen() {
    let socket = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        hook_prot::HOOK_PORT as u16,
    );
    let listener = TcpListener::bind(socket);
    if let Ok(listener) = listener {
        for socket in listener.incoming() {
            match socket {
                Ok(mut socket) => {
                    let mut data = Vec::new();
                    socket.read_to_end(&mut data).unwrap();
                    if HookProtocol::is_hook_protocol(data.clone()) {
                        let data: HookProtocol<serde_json::Value> = HookProtocol::new(data.clone());
                        let data = data.data;
                        dbg!(data);
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            }
        }
    }
}

fn send() {
    let dest = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        hook_prot::HOOK_PORT as u16,
    );
    let client = hook_client::HookClient::new(dest);
    let packet = hook_client::HookPacket::<serde_json::Value>::new(
        hook_prot::VERSION,
        hook_prot::HOOK_TYPE,
        serde_json::to_value(vec![1, 2, 3]).unwrap(),
    );
    client.send(packet).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).is_some() {
        listen();
    } else {
        send();
    }
}
