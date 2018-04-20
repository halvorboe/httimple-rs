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
    type Request = String;
    type Response = String;
    type Transport = Framed<T, Codec>;
    type BindTransport = Box<Future<Item = Self::Transport, Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(Codec);

        // The handshake requires that the client sends `You ready?`, so wait to
        // receive that line. If anything else is sent, error out the connection
        let handshake = transport.into_future()
            // If the transport errors out, we don't care about the transport
            // anymore, so just keep the error
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                // A line has been received, check to see if it is the handshake
                match line {
                    Some(ref msg) => {
                        if msg == PREFACE {
                            println!("SERVER: received client handshake");
                            // Send back the acknowledgement
                            Box::new(transport.send("Bring it!".to_string())) as Self::BindTransport
                        } else {
                            println!("{}", msg);
                            let err = io::Error::new(io::ErrorKind::Other, "invalid handshake");
                            Box::new(future::err(err)) as Self::BindTransport
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

