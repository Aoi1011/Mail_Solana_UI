use std::net::SocketAddr;

use tokio::prelude::*;
use failure;

pub struct Zookeeper {}

impl Zookeeper {
    pub fn connect(addr: &SocketAddr) -> impl Future<Item = Self, Error = failure::Error> {
        tokio::net::TcpStream::connect(addr).and_then(|stream| {
            Self::handshake(stream);
        })
    }

    fn handshake(stream: tokio::net::TcpStream) -> impl Future<Item = Self, Error = failure::Error> {
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

