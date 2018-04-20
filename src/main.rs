extern crate tokio_proto;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;
extern crate tokio_service;
extern crate bytes;
extern crate native_tls;
extern crate tokio_tls;

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
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future { // 
        println!("{:?}", req);
        future::ok(req)
    }
}


pub fn main() {
    // Create our TLS context through which new connections will be
    // accepted. This is where we pass in the certificate as well to
    // send to clients.
    // let der = include_bytes!(".cert/identity.p12");
    // let cert = Pkcs12::from_der(der, "mypass").unwrap();
    // let tls_cx = TlsAcceptor::builder(cert).unwrap()
    //                         .build().unwrap();

    // // Wrap up hyper's `Http` protocol in our own `Server` protocol. This
    // // will run hyper's protocol and then wrap the result in a TLS stream,
    // // performing a TLS handshake with connected clients.
    // let protocol = tokio_tls::proto::Server::new(Proto, tls_cx);

    // Finally use `tokio-proto`'s `TcpServer` helper struct to quickly
    // take our protocol above to running our hello-world Service on a
    // local TCP port.
    let addr = "127.0.0.1:1337".parse().unwrap();
    let srv = TcpServer::new(Proto, addr);
    println!("Listening on {}", addr);
    srv.serve(|| Ok(Hello));
}
