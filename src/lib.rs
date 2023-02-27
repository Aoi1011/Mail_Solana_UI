use std::net::SocketAddr;

use failure;
use proto::{request::Request, Packetizer};
use tokio::prelude::*;

mod proto;

pub struct ZooKeeper<S> {
    connection: Packetizer<S>,
}

impl<S> ZooKeeper<S> {
    pub fn connect(
        addr: &SocketAddr,
    ) -> impl Future<Item = ZooKeeper<tokio::net::TcpStream>, Error = failure::Error> {
        tokio::net::TcpStream::connect(addr)
            .map_err(failure::Error::from)
            .and_then(|stream| ZooKeeper::handshake(stream))
    }

    fn handshake(stream: S) -> impl Future<Item = Self, Error = failure::Error> {
        let request = Request::Connect {
            protocol_version: 0,
            last_zxid_seen: 0,
            timeout: 0,
            session_id: 0,
            passwd: vec![],
            read_only: false,
        };

        let mut zk = Packetizer::new(stream);
        let enqueuer = zk.enqueuer();
        tokio::spawn(zk);
        enqueuer.send(request).map(|(response, enqueuer)| {
            if response.is_none() {
                unimplemented!();
            }
            ZooKeeper { connection: enqueuer }
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let zk = tokio::run(ZooKeeper::connect());
//     }
// }
