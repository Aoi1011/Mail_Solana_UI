use std::collections::HashMap;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use failure::bail;
use futures::try_ready;
use tokio;
use tokio::prelude::*;

use self::request::{OpCode, Request};
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
    xid: usize,

    /// What operation are we waiting for a response for?
    /// keep xid, and where to send request
    reply: HashMap<i32, (OpCode, futures::unsync::oneshot::Sender<Request>)>,
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
            reply: Default::default(),
        }
    }

    pub fn inlen(&self) -> usize {
        self.inbox.len() - self.instart
    }

    pub fn outlen(&self) -> usize {
        self.outbox.len() - self.outstart
    }

    pub(crate) fn enqueue(
        &mut self,
        item: Request,
    ) -> impl Future<Item = Response, Error = failure::Error> {
        let (tx, rx) = futures::unsync::oneshot::channel();

        let lengthi = self.outbox.len();
        // dummy length
        self.outbox.push(0);
        self.outbox.push(0);
        self.outbox.push(0);
        self.outbox.push(0);

        let xid = self.xid + 1;
        self.xid += 1;
        self.reply.insert(xid as i32, (item.opcode(), tx));

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

        rx
    }

    fn poll_write(&mut self) -> Result<Async<()>, failure::Error> {
        while self.outlen() != 0 {
            let n = try_ready!(self.stream.write(&self.outbox[self.outstart..]));
            self.outstart += n;
            if self.outstart == self.outbox.len() {
                self.outbox.clear();
                self.outstart = 0;
            } else {
                return Ok(Async::NotReady);
            }
        }

        self.stream.poll_flush()
    }

    fn poll_read(&mut self) -> Result<Async<()>, failure::Error> {
        loop {
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

            let buf = &self.inbox[self.instart..self.instart + need];
            self.instart += 4;
            let xid = buf.inbox[buf.instart..].read_i32::<BigEndian>()?;
            self.instart += 4;

            // find the waiting request future
            let Some((opcode, tx)) = self.reply.remove(xid); // return an error if xid was unknown

            let r = Response::parse(
                opcode,
                &self.inbox[self.instart..(self.instart + need - 4 - 4)],
            )?;
            self.instart += need - 4 - 4;
            tx.send(r);

            if self.instart == self.inbox.len() {
                self.inbox.clear();
                self.instart = 0;
            }
            Ok(Async::Ready(()))
        }
    }
}

impl<S> Future for Packetizer<S>
where
    S: AsyncRead + AsyncWrite,
{
    type Item = ();
    type Error = failure::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let r = self.poll_read()?;
        let w = self.poll_write()?;

        match (r, w) {
            (Async::Ready(()), Async::Ready(())) => Ok(Async::Ready(())),
            (Async::Ready(()), _) => bail!("outstandig requests, but response channel closed."),
            _ => Ok(Async::NotReady),
        }
    }
}
