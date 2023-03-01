use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use failure;

use crate::{proto::request::OpCode, types::Stat};

#[derive(Debug)]
pub(crate) enum Response {
    Connect {
        protocol_version: i32,
        timeout: i32,
        session_id: i64,
        passwd: Vec<u8>,
        read_only: bool,
    },
    Exists {
        stat: Stat,
    },
}

pub trait ReadFrom: Sized {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self>;
}

impl ReadFrom for Stat {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Stat> {
        Ok(Stat {
            czxid: read.read_i64::<BigEndian>()?,
            mzxid: read.read_i64::<BigEndian>()?,
            ctime: read.read_i64::<BigEndian>()?,
            mtime: read.read_i64::<BigEndian>()?,
            version: read.read_i32::<BigEndian>()?,
            cversion: read.read_i32::<BigEndian>()?,
            aversion: read.read_i32::<BigEndian>()?,
            ephemeral_owner: read.read_i64::<BigEndian>()?,
            data_length: read.read_i32::<BigEndian>()?,
            num_children: read.read_i32::<BigEndian>()?,
            pzxid: read.read_i64::<BigEndian>()?,
        })
    }
}

pub trait BufferReader: Read {
    fn read_buffer(&mut self) -> io::Result<Vec<u8>>;
}

impl<R: Read> BufferReader for R {
    fn read_buffer(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read_i32::<BigEndian>()?;
        let len = if len < 0 { 0 } else { len as usize };
        let mut buf = vec![0; len];
        let read = self.read(&mut buf)?;
        if read == len {
            Ok(buf)
        } else {
            Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "read_buffer failed",
            ))
        }
    }
}

pub trait StringReader: Read {
    fn read_string(&mut self) -> io::Result<String>;
}

impl<R: Read> StringReader for R {
    fn read_string(&mut self) -> io::Result<String> {
        let raw = self.read_buffer()?;
        Ok(String::from_utf8(raw).unwrap())
    }
}

impl Response {
    pub(super) fn parse(opcode: OpCode, buf: &[u8]) -> Result<Self, failure::Error> {
        let mut reader = buf;
        match opcode {
            OpCode::CreateSession => Ok(Response::Connect {
                protocol_version: reader.read_i32::<BigEndian>()?,
                timeout: reader.read_i32::<BigEndian>()?,
                session_id: reader.read_i64::<BigEndian>()?,
                passwd: reader.read_buffer()?,
                read_only: reader.read_u8()? != 0,
            }),
            OpCode::Exists => Ok(Response::Exists {
                stat: Stat::read_from(&mut reader)?,
            }),
            _ => {
                unimplemented!()
            }
        }
    }
}
