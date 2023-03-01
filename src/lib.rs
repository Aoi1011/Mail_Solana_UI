use std::net::SocketAddr;

use failure::{self, bail};
use proto::{error::ZkError, request::Request, Enqueuer, Packetizer};
use tokio::prelude::*;
use types::Stat;

mod proto;
mod types;

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
        eprintln!("about to handshake");

        let enqueuer = Packetizer::new(stream);
        enqueuer.enqueue(request).map(move |response| {
            eprintln!("{:?}", response);
            ZooKeeper {
                connection: enqueuer,
            }
        })
    }

    // TODO: want structured error type
    fn exists(&self, path: &str) -> impl Future<Item = Option<Stat>, Error = failure::Error> {
        self.connection
            .enqueue(proto::request::Request::Exists {
                path: path.to_string(),
                watch: 0,
            })
            .and_then(|r| match r {
                Ok(proto::response::Response::Exists { stat }) => Ok(Some(stat)),
                Err(ZkError::NoNode) => Ok(None),
                Err(e) => bail!("exits call failed {:?}", e),
                _ => unreachable!(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let zk = rt.block_on(
            ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap()).and_then(|zk| {
                zk.exists("/foo")
                    .inspect(|stat| eprintln!("exists? {:?}", stat))
            }),
        );
        drop(zk);
        rt.shutdown_on_idle();
    }
}
