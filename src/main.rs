extern crate bytes;
extern crate mio;
extern crate bitreader;
extern crate hpack;
extern crate rustls;

mod proto;
mod codec;
mod frame; 
mod config;
mod connection;
mod server;
mod session;

use mio::tcp::{TcpListener};

use std::fs;
use std::net;


use server::Server;

// Token for our listening socket.
const LISTENER: mio::Token = mio::Token(0);

fn main() {

    let mut addr: net::SocketAddr = "0.0.0.0:3000".parse().unwrap();
    addr.set_port(3000);

    let config = config::make_config();

    let listener = TcpListener::bind(&addr).expect("cannot listen on port");
    let mut poll = mio::Poll::new()
        .unwrap();
    poll.register(&listener,
                  LISTENER,
                  mio::Ready::readable(),
                  mio::PollOpt::level())
        .unwrap();

    println!("Starting server");
    let mut tlsserv = Server::new(listener, config);

    let mut events = mio::Events::with_capacity(256);

    let mut count = 0;

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                LISTENER => {
                    count += 1;
                    println!("[CONNECTION] {}", count);
                    if !tlsserv.accept(&mut poll) {
                        break;
                    }
                }
                _ => {
                    tlsserv.conn_event(&mut poll, &event)
                }
            }
        }
    }
}
