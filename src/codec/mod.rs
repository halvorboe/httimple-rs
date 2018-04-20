use tokio_io::codec::{Encoder, Decoder};
use std;
use bytes::BytesMut;

use std::io;

pub struct Codec;

impl Encoder for Codec {
    type Item = String;
    type Error = io::Error;
    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }

}

impl Decoder for Codec {
  type Item = String;
  type Error = io::Error;

  fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
      // 1. Read head 
      // 2. Read from head
      if let Some(i) = buf.iter().position(|&b| b == b'\n') {
          // remove the serialized frame from the buffer.
          let line = buf.split_to(i);

          // Also remove the '\n'
          buf.split_to(1);

          // Turn this data into a UTF string and return it in a Frame.
          match std::str::from_utf8(&line) {
              Ok(s) => Ok(Some(s.to_string())),
              Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                           "invalid UTF-8")),
          }
      } else {
          Ok(None)
      }
  }
}