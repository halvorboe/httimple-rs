use std::io;
use codec::Codec;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Framed};
use tokio_proto::pipeline::ServerProto;
use future;
use future::Future;
use futures::Sink;
use futures::Stream;

pub const PREFACE : &str = "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

pub struct Proto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Proto {
    type Request = Vec<u8>;
    type Response = Vec<u8>;

    /// `Framed<T, LineCodec>` is the return value of `io.framed(LineCodec)`
    type Transport = Framed<T, Codec>;
    type BindTransport = Box<Future<Item = Self::Transport, Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(Codec);
        let handshake = transport.into_future()
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                match line {
                    Some(ref msg) => {
                        println!("{}", msg.len());
                        let line = String::from_utf8(msg[0..24].to_vec());
                        match line {
                            Ok(msg) => {
                                if  msg == PREFACE {
                                    println!("SERVER: received client handshake");
                                    // Send settings frame
                                    Box::new(transport.send(Vec::new())) as Self::BindTransport
                                } else {
                                    println!("Invalid");
                                    let err = io::Error::new(io::ErrorKind::Other, "invalid handshake");
                                    Box::new(future::err(err)) as Self::BindTransport
                                }
                            },
                            Err(_) => {
                                println!("Failed string parse");
                                let e = io::Error::new(io::ErrorKind::Other, "Failed string parse");
                                Box::new(future::err(e)) as Self::BindTransport
                            }
                        }                        
                    }
                    _ => {
                        // The client sent an unexpected handshake, error out
                        // the connection
                        println!("SERVER: client handshake INVALID:");
                        let err = io::Error::new(io::ErrorKind::Other, "invalid handshake");
                        Box::new(future::err(err)) as Self::BindTransport
                    }
                }
            });

        Box::new(handshake)
    }
}
