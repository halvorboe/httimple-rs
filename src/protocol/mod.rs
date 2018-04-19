

use std;
use std::io;


use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Framed, Encoder, Decoder};
use bytes::BytesMut;
use tokio_proto::pipeline::ServerProto;

pub struct LineCodec;

impl Encoder for LineCodec {
  type Item = String;
  type Error = io::Error;

  fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
      buf.extend(msg.as_bytes());
      buf.extend(b"\n");
      Ok(())
  }
}

impl Decoder for LineCodec {
  type Item = String;
  type Error = io::Error;

  fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
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

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

