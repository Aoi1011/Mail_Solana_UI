use std::net::SocketAddr;

use failure;
use proto::{request::Request, Enqueuer, Packetizer};
use tokio::prelude::*;

mod proto;

pub struct ZooKeeper {
    connection: Enqueuer,
}

impl ZooKeeper {
    pub fn connect(addr: &SocketAddr) -> impl Future<Item = Self, Error = failure::Error> {
        tokio::net::TcpStream::connect(addr)
            .map_err(failure::Error::from)
            .and_then(|stream| Self::handshake(stream))
    }

    fn handshake<S>(stream: S) -> impl Future<Item = Self, Error = failure::Error>
    where
        S: 'static + Send + AsyncRead + AsyncWrite,
    {
        let request = Request::Connect {
            protocol_version: 0,
            last_zxid_seen: 0,
            timeout: 0,
            session_id: 0,
            passwd: vec![],
            read_only: false,
        };

        let enqueuer = Packetizer::new(stream);
        enqueuer.enqueue(request).map(move |response| ZooKeeper {
            connection: enqueuer,
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
