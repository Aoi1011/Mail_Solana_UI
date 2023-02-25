use byteorder::{BigEndian, WriteBytesExt};
use tokio;
use tokio::prelude::*;

pub struct Packetizer<S> {
    stream: S,

    /// Bytes we have not yet set
    outbox: Vec<u8>,

    /// Prefix of outbox that has been set
    outstart: usize,

    /// Bytes we have not yet deserialized
    inbox: Vec<u8>,

    /// Prefix of outbox that has been set
    instart: usize,

    /// Connection ID for the wrapped stream
    xid: i32,
}

impl<S> Packetizer<S> {
    pub(crate) fn new(stream: S) -> Packetizer<S> {
        Packetizer {
            stream,
            outbox: Vec::new(),
            outstart: 0,
            inbox: Vec::new(),
            instart: 0,
            xid: 0,
        }
    }
}

pub(crate) enum ZookeeperRequest {
    ConnectRequest {
        protocol_version: i32,
        last_zxid_seen: i64,
        timeout: i32,
        session_id: i64,
        passwd: Vec<u8>,
        read_only: bool,
    },
}

impl ZookeeperRequest {
    fn serialize_into(&self, buffer: &mut Vec<u8>) -> usize {
        unimplemented!()
    }
}

impl<S> Sink for Packetizer<S> {
    type SinkItem = ZookeeperRequest;
    type SinkError = failure::Error;

    fn start_send(
        &mut self,
        item: Self::SinkItem,
    ) -> futures::StartSend<Self::SinkItem, Self::SinkError> {
        let lengthi = self.outbox.len();
        // dummy length
        self.outbox.push(0);
        self.outbox.push(0);
        self.outbox.push(0);
        self.outbox.push(0);

        // xid
        self.outbox
            .write_i32::<BigEndian>(self.xid)
            .expect("Vec::write should never fail");

        // type and payload
        let n = item.serialize_into(&mut self.outbox);

        // set true length
        let length = &mut self.outbox[lengthi..lengthi + 4];
        length
            .write_i32::<BigEndian>(0)
            .expect("Vec::write should never fail");
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        let n = try_ready!(self.stream.write(&self.outbox[self.outstart..]));
        self.outstart += n;
        if self.outstart == self.outbox.len() {
            self.outbox.clear();
            self.outstart = 0;
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}
