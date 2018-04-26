pub mod call;
pub mod message;
mod connection;
mod config;

use std::sync::Arc;
use mio;
use self::connection::Connection;
use std::collections::HashMap;
use mio::tcp::{TcpListener};
use rustls;
use self::call::Call;
use self::message::Message;

use std::net;

type Callback = fn(&Call) -> Message;
const LISTENER: mio::Token = mio::Token(0);

/// 
/// This is how you use it:
/// 
/// ```
/// let mut app = App::new();
/// app.serve("/", | call: &Call | -> Message {
///   Message::from(file("index.html"))
/// });
/// app.start();
/// ```
/// 
pub struct App {
    server: TcpListener,
    connections: HashMap<mio::Token, Connection>,
    next_id: usize,
    tls_config: Arc<rustls::ServerConfig>,
    handler: HashMap<Vec<u8>, Callback>,
}

impl App {
    pub fn new() -> App {
        println!("[SETUP] Setting up socket...");
        let mut addr: net::SocketAddr = "0.0.0.0:3000".parse().unwrap();
        use std::env;

        let key = "PORT";
        match env::var(key) {
            Ok(val) => addr.set_port(val.parse().unwrap()),
            Err(_) => {
                addr.set_port(3000);
                println!("Failed to find port");
            }
        }
        

        println!("[SETUP] Getting config...");
        let config = config::make_config();

        let listener = TcpListener::bind(&addr).expect("cannot listen on port");

        App {
            server: listener,
            connections: HashMap::new(),
            next_id: 2,
            tls_config: config,
            handler: HashMap::new(),
        }
    }

    fn accept(&mut self, poll: &mut mio::Poll) -> bool {
        match self.server.accept() {
            Ok((socket, addr)) => {
                println!("Accepting new connection from {:?}", addr);

                let tls_session = rustls::ServerSession::new(&self.tls_config);
                let handler = self.handler.clone();

                println!("--------------------------X");
                for (key, _) in &self.handler {
                    println!("{:?}", String::from_utf8(key.clone()).unwrap());
                }
                println!("--------------------------X");

                let token = mio::Token(self.next_id);
                self.next_id += 1;

                self.connections.insert(token, Connection::new(socket, token, tls_session, handler));
                self.connections[&token].register(poll);
                true
            }
            Err(e) => {
                println!("encountered error while accepting connection; err={:?}", e);
                false
            }
        }
    }

    fn conn_event(&mut self, poll: &mut mio::Poll, event: &mio::Event) {
        let token = event.token();

        if self.connections.contains_key(&token) {
            self.connections
                .get_mut(&token)
                .unwrap()
                .ready(poll, event);

            if self.connections[&token].is_closed() {
                self.connections.remove(&token);
            }
        }
    }

    ///
    /// Routes calls comming from that path to that function.
    /// 

    pub fn serve(&mut self, path: &'static str, callback: Callback) {
        self.handler.insert(String::from(path).as_bytes().to_vec(), callback);
    }

    ///
    /// Boot up the server
    /// 

    pub fn start(&mut self) {

        println!("[BOOT] Server started 😄");

        let mut poll = mio::Poll::new()
            .unwrap();
        poll.register(&self.server,
                    LISTENER,
                    mio::Ready::readable(),
                    mio::PollOpt::level())
            .unwrap();
        let mut events = mio::Events::with_capacity(256);

        let mut count = 0;

        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    LISTENER => {
                        count += 1;
                        println!("[CONNECTION] {}", count);
                        if !self.accept(&mut poll) {
                            break;
                        }
                    }
                    _ => {
                        self.conn_event(&mut poll, &event)
                    }
                }
            }
        }
    } 

}

