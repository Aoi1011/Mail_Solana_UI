use std::net::SocketAddr;

use failure;
use proto::Packetizer;
use tokio::prelude::*;

mod proto;

pub struct Zookeeper<S> {
    connection: Packetizer<S>,
}

impl Zookeeper<S> {
    pub fn connect(addr: &SocketAddr) -> impl Future<Item = Self, Error = failure::Error> {
        tokio::net::TcpStream::connect(addr).and_then(|stream| {
            Self::handshake(stream);
        })
    }

    fn handshake(
        stream: tokio::net::TcpStream,
    ) -> impl Future<Item = Self, Error = failure::Error> {
        let request = proto::Request::Connect {
            protocol_version: 0,
            last_zxid_seen: 0,
            timeout: 0,
            session_id: 0,
            passwd: vec![],
            read_only: false,
        };
        Packetizer::new(stream)
            .send(request)
            .and_then(|zk| {
                zk.into_future();
            })
            .map(|(response, zk)| {
                if response.is_none() {
                    unimplemented!();
                }
                Zookeeper { connection: zk }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let zk = tokio::run(Zookeeper::connect());
    }
}
