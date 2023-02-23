use std::net::SocketAddr;

use tokio::prelude::*;
use failure;

mod proto;

pub struct Zookeeper {}

impl Zookeeper {
    pub fn connect(addr: &SocketAddr) -> impl Future<Item = Self, Error = failure::Error> {
        tokio::net::TcpStream::connect(addr).and_then(|stream| {
            Self::handshake(stream);
        })
    }

    fn handshake(stream: tokio::net::TcpStream) -> impl Future<Item = Self, Error = failure::Error> {
        let request = proto::Connection {};
        let stream = proto::wrap(stream);
        stream.send(request).and_then(|stream| {
            stream.receive();
        }).and_then(|(response, stream)| {
                Zookeeper {}
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

