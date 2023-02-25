use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use failure::bail;
use futures::try_ready;
use tokio;
use tokio::prelude::*;

use self::request::{Request, OpCode};
use self::response::Response;

pub mod request;
pub mod response;

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

    /// What operation are we waiting for a response for?
    last_sent: Option<OpCode>,
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
            last_sent: None,
        }
    }

    pub fn inlen(&self) -> usize {
        self.inbox.len() - self.instart
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
        } else {
            return Ok(Async::NotReady);
        }
        self.stream.poll_flush()
    }
}

impl<S> Stream for Packetizer<S>
where
    S: AsyncRead,
{
    type Item = Response;
    type Error = failure::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut need = if self.inlen() > 4 {
            let length = self.inbox[self.instart..].read_i32::<BigEndian>()?;
            length + 4
        } else {
            4
        };

        while self.inlen() < need {
            let read_from = self.inbox.len();
            self.inbox.resize(read_from + need, 0);
            match self.stream.poll_read(&mut self.inbox[read_from..])? {
                Async::Ready(n) => {
                    if n == 0 {
                        if self.inlen() != 0 {
                            bail!(
                                "connection closed with {} bytes left in buffer",
                                self.inlen()
                            );
                        } else {
                            return Ok(Async::Ready(None));
                        }
                    }
                    self.inbox.truncate(read_from + n);
                    if self.inlen() > 4 && need != 4 {
                        let length = self.inbox[self.instart..].read_i32::<BigEndian>()?;
                        need + length
                    }
                }
                Async::NotReady => {
                    self.inbox.truncate(read_from);
                    return Ok(Async::NotReady);
                }
            }
        }

        self.instart += 4;
        let r = Response::parse(&self.inbox[self.instart..(self.instart + need - 4)])?;
        self.instart += need - 4;

        if self.instart == self.inbox.len() {
            self.inbox.clear();
            self.instart = 0;
        }

        Ok(Async::Ready(Some(r)))
    }
}
