extern crate tokio_proto;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;
extern crate tokio_service;
extern crate bytes;
extern crate native_tls;
extern crate tokio_tls;
extern crate bitreader;

mod proto;
mod codec;
mod frame; 

use std::io;

use proto::Proto;

use futures::future;
use futures::future::FutureResult;
use tokio_proto::TcpServer;
use tokio_service::Service;
use futures::future::{ok, Future};
use native_tls::{TlsAcceptor, Pkcs12};



struct Hello;

impl Service for Hello {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future { 
        let line = String::from_utf8(req.clone());
        match line {
            Ok(msg) => {
                println!("[R] {}", msg)
            },
            Err(err) => {
                println!("[Error] {}", err)
            }
        }
        future::ok(req)
    }
}


pub fn main() {
    // let der = include_bytes!(".cert/identity.p12");
    // let cert = Pkcs12::from_der(der, "mypass").unwrap();
    // let tls_cx = TlsAcceptor::builder(cert).unwrap()
    //                         .build().unwrap();

    // let protocol = tokio_tls::proto::Server::new(Proto, tls_cx);

    let addr = "127.0.0.1:1337".parse().unwrap();
    let srv = TcpServer::new(Proto, addr);
    println!("Listening on {}", addr);
    srv.serve(|| Ok(Hello));
}
