use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use failure::{self, bail};

use super::request::OpCode;

pub(crate) enum Response {
    ConnectResponse {
        protocol_version: i32,
        timeout: i32,
        session_id: i64,
        passwd: Vec<u8>,
        read_only: bool,
    },
}

pub trait BufferReader: Read {
    fn read_buffer(&mut self) -> Result<Vec<u8>, failure::Error>;
}

impl<R: Read> BufferReader for R {
    fn read_buffer(&mut self) -> Result<Vec<u8>, failure::Error> {
        let len = self.read_i32::<BigEndian>()?;
        let len = if len < 0 {
            0
        } else {
            len as usize
        };
        let mut buf = vec![0; len];
        let read = self.read(&mut buf)?;
        if read == len {
            Ok(buf)
        } else {
            bail!("read_buffer failed")
        }
    }
}

impl Response {
    pub(super) fn parse(opcode: OpCode, buf: &[u8]) -> Result<Self, failure::Error> {
        let mut reader = buf;
        match opcode {
            OpCode::Connect => Ok(Response::ConnectResponse {
                protocol_version: reader.read_i32::<BigEndian>()?,
                timeout: reader.read_i32::<BigEndian>()?,
                session_id: reader.read_i64::<BigEndian>()?,
                passwd: reader.read_buffer()?,
                read_only: reader.read_u8()? != 0,
            }),
            _ => {
                unimplemented!()
            }
        }
    }
}
