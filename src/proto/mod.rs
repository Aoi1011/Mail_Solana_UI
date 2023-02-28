use std::collections::HashMap;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use failure::{bail, format_err};
use futures::try_ready;
use futures::{
    future::Either,
    sync::{mpsc, oneshot},
};
use tokio;
use tokio::prelude::*;

use self::request::{OpCode, Request};
use self::response::Response;

pub mod request;
pub mod response;

pub(crate) struct Enqueuer(mpsc::UnboundedSender<(Request, oneshot::Sender<Response>)>);

impl Enqueuer {
    pub(crate) fn enqueue(
        &self,
        req: Request,
    ) -> impl Future<Item = Response, Error = failure::Error> {
        let (tx, rx) = oneshot::channel();
        match self.0.unbounded_send((req, tx)) {
            Ok(()) => {
                Either::A(rx.map_err(|e| format_err!("failed to enqueue new request: {:?}", e)))
            }
            Err(e) => {
                Either::B(Err(format_err!("failed to enqueue new request: {:?}", e))).into_future()
            }
        }
    }
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

    /// What operation are we waiting for a response for?
    /// keep xid, and where to send request
    reply: HashMap<i32, (OpCode, oneshot::Sender<Response>)>,

    /// Incoming requests
    rx: mpsc::UnboundedReceiver<(Request, oneshot::Sender<Response>)>,

    /// Next xid to issue
    xid: i32,

    exiting: bool,
}

impl<S> Packetizer<S> {
    // TODO: document that it calls tokio::spawn
    pub(crate) fn new(stream: S) -> Enqueuer
    where
        S: 'static + Send + AsyncRead + AsyncWrite,
    {
        let (tx, rx) = mpsc::unbounded();
        tokio::spawn(
            Packetizer {
                stream,
                outbox: Vec::new(),
                outstart: 0,
                inbox: Vec::new(),
                instart: 0,
                xid: 0,
                reply: Default::default(),
                rx,
                exiting: false,
            }
            .map_err(|e| {
                // TODO: expose this error to the user somehow
                drop(e);
            }),
        );

        Enqueuer(tx)
    }
}

impl<S> Packetizer<S> {
    pub fn outlen(&self) -> usize {
        self.outbox.len() - self.outstart
    }

    pub fn inlen(&self) -> usize {
        self.inbox.len() - self.instart
    }

    pub(crate) fn poll_enqueue(&mut self) -> Result<Async<()>, ()> {
        loop {
            let (item, tx) = match try_ready!(self.rx.poll()) {
                Some((item, tx)) => (item, tx),
                None => return Err(()),
            };

            let lengthi = self.outbox.len();
            // dummy length
            self.outbox.push(0);
            self.outbox.push(0);
            self.outbox.push(0);
            self.outbox.push(0);

            let xid = self.xid;
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
            let mut length = &mut self.outbox[lengthi..lengthi + 4];
            length
                .write_i32::<BigEndian>(written as i32)
                .expect("Vec::write should never fail");
        }
    }
}

impl<S> Packetizer<S>
where
    S: AsyncRead + AsyncWrite,
{
    fn poll_write(&mut self) -> Result<Async<()>, failure::Error>
    where
        S: AsyncWrite,
    {
        while self.outlen() != 0 {
            let n = try_ready!(self.stream.poll_write(&self.outbox[self.outstart..]));
            self.outstart += n;
            if self.outstart == self.outbox.len() {
                self.outbox.clear();
                self.outstart = 0;
            } else {
                return Ok(Async::NotReady);
            }
        }

        self.stream.poll_flush().map_err(failure::Error::from)
    }

    fn poll_read(&mut self) -> Result<Async<()>, failure::Error>
    where
        S: AsyncRead,
    {
        loop {
            let mut need = if self.inlen() > 4 {
                let length = (&mut &self.inbox[self.instart..]).read_i32::<BigEndian>()?;
                length + 4
            } else {
                4
            };

            while self.inlen() < need as usize {
                let read_from = self.inbox.len();
                self.inbox.resize(read_from + need as usize, 0);
                match self.stream.poll_read(&mut self.inbox[read_from..])? {
                    Async::Ready(n) => {
                        if n == 0 {
                            if self.inlen() != 0 {
                                bail!(
                                    "connection closed with {} bytes left in buffer",
                                    self.inlen()
                                );
                            } else {
                                return Ok(Async::Ready(()));
                            }
                        }
                        self.inbox.truncate(read_from + n);
                        if self.inlen() > 4 && need != 4 {
                            let length =
                                (&mut &self.inbox[self.instart..]).read_i32::<BigEndian>()?;
                            need += length;
                        }
                    }
                    Async::NotReady => {
                        self.inbox.truncate(read_from);
                        return Ok(Async::NotReady);
                    }
                }
            }

            {
                let mut buf = &self.inbox[self.instart..self.instart + need as usize];
                let length = buf.read_i32::<BigEndian>()?;
                let xid = buf.read_i32::<BigEndian>()?;

                // find the waiting request future
                let (opcode, tx) = self.reply.remove(&xid).unwrap(); // return an error if xid was unknown

                let r = Response::parse(opcode, buf)?;
                self.instart += need as usize;
                tx.send(r);
            }

            if self.instart == self.inbox.len() {
                self.inbox.clear();
                self.instart = 0;
            }
            return Ok(Async::Ready(()));
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
        if !self.exiting {
            match self.poll_enqueue() {
                Ok(_) => {}
                Err(()) => {
                    // no more requests will be enqueued
                    self.exiting = true;
                }
            }
        }

        let r = self.poll_read()?;
        let w = self.poll_write()?;

        match (r, w) {
            (Async::Ready(()), Async::Ready(())) if self.exiting => Ok(Async::Ready(())),
            (Async::Ready(()), Async::Ready(())) => Ok(Async::NotReady),
            (Async::Ready(()), _) => bail!("outstandig requests, but response channel closed."),
            (_, Async::Ready(())) if self.exiting => {
                // TODO: send OpCode::CloseSession
                Ok(Async::NotReady)
            }
            _ => Ok(Async::NotReady),
        }
    }
}
