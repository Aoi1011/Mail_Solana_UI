use byteorder::{BigEndian, WriteBytesExt};
use tokio;
use tokio::prelude::*;

use std::io::{self};

#[repr(i32)]
pub enum OpCode {
    Notification = 0,
    Create = 1,
    Delete = 2,
    Exists = 3,
    GetData = 4,
    SetData = 5,
    GetACL = 6,
    SetACL = 7,
    GetChildren = 8,
    Synchronize = 9,
    Ping = 11,
    GetChildren2 = 12,
    Check = 13,
    Multi = 14,
    Auth = 100,
    SetWatches = 101,
    Sasl = 102,
    CreateSession = -10,
    CloseSession = -11,
    Error = -1,
}

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

pub(crate) enum Request {
    Connect {
        protocol_version: i32,
        last_zxid_seen: i64,
        timeout: i32,
        session_id: i64,
        passwd: Vec<u8>,
        read_only: bool,
    },
}

impl Request {
    fn serialize_into(&self, buffer: &mut Vec<u8>) -> Result<(), io::Error> {
        let mut n = 0;
        match *self {
            Request::Connect {
                protocol_version,
                last_zxid_seen,
                timeout,
                session_id,
                ref passwd,
                read_only,
            } => {
                // buffer.write_i32(OpCode::Auth);
                // n += 4;
                buffer.write_i32::<BigEndian>(protocol_version)?;
                buffer.write_i64::<BigEndian>(last_zxid_seen)?;
                buffer.write_i32::<BigEndian>(timeout)?;
                buffer.write_i64::<BigEndian>(session_id)?;
                buffer.write_i32::<BigEndian>(passwd.len() as i32)?;
                buffer.write_all(passwd)?;
                buffer.write_u8(read_only as u8)?;
                Ok(())
            }
        }
    }
}

impl<S> Sink for Packetizer<S> {
    type SinkItem = Request;
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

        if let Request::Connect { .. } = item {
        } else {
            // xid
            self.outbox
                .write_i32::<BigEndian>(self.xid)
                .expect("Vec::write should never fail");
        }

        // type and payload
        item.serialize_into(&mut self.outbox)
            .expect("Vec::Write should never fail");

        // set true length
        let written = self.outbox.len() - lengthi - 4;
        let length = &mut self.outbox[lengthi..lengthi + 4];
        length
            .write_i32::<BigEndian>(written as i32)
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
