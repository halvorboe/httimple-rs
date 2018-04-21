use tokio_io::codec::{Encoder, Decoder};
use std;
use bytes::{Bytes, BytesMut};
use frame::Frame;
use std::io;

pub struct Codec;

impl Encoder for Codec {
    type Item = Vec<u8>;
    type Error = io::Error;
    fn encode(&mut self, msg: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg);
        Ok(())
    }

}

impl Decoder for Codec {
  type Item = Vec<u8>;
  type Error = io::Error;

  fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Vec<u8>>> {
        println!("{}", buf.len());
        if buf.len() > 9 {
            let b = Bytes::from(buf.to_vec());
            Frame::from(b);
        }
        if buf.len() > 0 {
            let l = buf.len();
            let s = buf.split_to(l);
            Ok(Some(s.to_vec()))
        } else {
            Ok(None)
        }
      // 1. Read head 
      // 2. Read from head
      // 3. Read with type and flag in mind 
      // 4. Stop when length (specified in header is reached)
    //   if let Some(i) = buf.iter().map(|x| x).collect() {
    //       // remove the serialized frame from the buffer.
    //       let line = buf.split_to(i);

    //       // Also remove the '\n'
    //       buf.split_to(1);

    //       // Turn this data into a UTF string and return it in a Frame.
    //       match std::str::from_utf8(&line) {
    //           Ok(s) => Ok(Some((0, s.to_string()))),
    //           Err(_) => Err(io::Error::new(io::ErrorKind::Other,
    //                                        "invalid UTF-8")),
    //       }
    //   } else {
    //       Ok(None)
    //   }
  }
}