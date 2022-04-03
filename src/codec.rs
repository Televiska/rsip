use std::convert::TryInto;
use std::{convert::TryFrom, io::Write};

use bytes::{BytesMut, Bytes, BufMut};
use tokio_util::codec;

use crate::{SipMessage, error, Request};
use crate::message::request::Tokenizer;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SipServer();

impl SipServer {
    pub fn new() -> Self {
        SipServer()
    }
}

#[derive(Debug)]
pub enum MessageError {
    SipDecodeError(crate::error::Error),

    IoError(std::io::Error),
}
impl From<std::io::Error> for MessageError {
    fn from(err: std::io::Error) -> Self {
        MessageError::IoError(err)
    }
}

impl<E> From<E> for MessageError 
    where E : Into<error::Error> {
    fn from(err: E) -> Self {
        MessageError::SipDecodeError(err.into())
    }
}

impl codec::Decoder for SipServer {
    type Item = Request;

    type Error = MessageError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let bytes = src.clone().freeze();
        { // debugging
            let s = String::from_utf8_lossy(&bytes[..]);
            println!("recv bytes:");
            println!("  -----");
            for line in s.split("\r\n") {
                println!("  |   {}", line);
            }
            println!("  ----- empty: {}", bytes.is_empty());
        }
        if bytes.is_empty() {
            // println!("msg is empty");
            src.clear();
            return Ok(None)
        }
        if bytes == "\r\n\r\n".as_bytes() {
            // something weird has happened: 
            // somehow the end sequence was received even though no message preceded it.
            // This happens with some clients that send this sequence to keep a (tcp) connection alive
            // => clear the end sequence and tell tokio that we did not receive a full message
            src.clear();
            return Ok(None)
        }
        // println!("msg is not empty");
        let res = match Tokenizer::tokenize(&bytes) {
            Ok((empty_rem_tokens, tok)) => {
                assert_eq!(empty_rem_tokens, []);
                src.clear();
                let req : Request = tok.try_into()?;
                Ok(Some(req))
            },
            Err(nom::Err::Incomplete(_)) => {
                println!("nom says 'incomplete!'");
                Ok(None)
            },
            Err(err)=> {
                Err(err.into())
            },
        };
        // println!("parsed into: {:?}", res);
        res
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use bytes::{BytesMut, BufMut};
    use tokio_util::codec::Decoder;

    use super::SipServer;

    #[test]
    fn test_decode() {
        let msg = "REGISTER asd.com SIP/2.0\r\nMY-HEADER: my-value\r\n\r\n";
        let mut buffer: BytesMut = msg.into();
        let res = SipServer::new().decode(&mut buffer);
        println!("decode single");
        println!("{:?}", res);
    }

    #[test]
    fn test_decode_multiple() {
        let msg = "REGISTER asd.com SIP/2.0\r\nMY-HEADER: my-value\r\n\r\n";
        let mut buffer: BytesMut = msg.into();
        let mut codec = SipServer::new();
        let res = codec.decode(&mut buffer);
        println!("{:?}", res);
        
        let msg2 = "REGISTER second.com SIP/2.0\r\nSND-MY-HEADER: my-snd-value\r\n\r\n";
        let mut buffer2 = msg2.into();
        let res = codec.decode(&mut buffer2);
        println!("decode second {:?}", res);
    }
}

impl codec::Encoder<SipMessage> for SipServer {
    type Error = std::io::Error;

    fn encode(&mut self, item: SipMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes: Bytes = item.into();
        dst.writer().write(&bytes[..])?;
        Ok(())
    }
}
