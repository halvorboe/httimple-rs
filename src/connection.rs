use mio::tcp::{TcpStream, Shutdown};
use rustls;
use rustls::Session;
use mio;
use std::io;
use std::io::{Read, Write};
use std::collections::HashMap;

use session;
use proto;
use codec;
use frame::Frame;
use frame::headers::Headers;

use frame::head::Head;
use frame::data::Data;
use frame::settings::Settings;

pub struct Connection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    h2_session: session::Session,
    tls_session: rustls::ServerSession,
    back: Option<TcpStream>,
    sent_http_response: bool,
}

/// Open a plaintext TCP-level connection for forwarded connections.
fn open_back() -> Option<TcpStream> {
    None
}


/// This used to be conveniently exposed by mio: map EWOULDBLOCK
/// errors to something less-errory.
fn try_read(r: io::Result<usize>) -> io::Result<Option<usize>> {
    match r {
        Ok(len) => Ok(Some(len)),
        Err(e) => {
            if e.kind() == io::ErrorKind::WouldBlock {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

impl Connection {
    pub fn new(socket: TcpStream,
           token: mio::Token,
           tls_session: rustls::ServerSession)
           -> Connection {
        let back = open_back();
        Connection {
            socket: socket,
            token: token,
            closing: false,
            closed: false,
            h2_session: session::Session { accepted: false, settings: HashMap::new(), streams: HashMap::new() },
            tls_session: tls_session,
            back: back,
            sent_http_response: false,
        }
    }

    /// We're a connection, and we have something to do.
    pub fn ready(&mut self, poll: &mut mio::Poll, ev: &mio::Event) {
        // If we're readable: read some TLS.  Then
        // see if that yielded new plaintext.  Then
        // see if the backend is readable too.
        if ev.readiness().is_readable() {
            self.do_tls_read();
            self.try_plain_read();
            self.try_back_read();
        }

        if ev.readiness().is_writable() {
            self.do_tls_write();
        }

        if self.closing && !self.tls_session.wants_write() {
            let _ = self.socket.shutdown(Shutdown::Both);
            self.close_back();
            self.closed = true;
        } else {
            self.reregister(poll);
        }
    }

    /// Close the backend connection for forwarded sessions.
    pub fn close_back(&mut self) {
        if self.back.is_some() {
            let back = self.back.as_mut().unwrap();
            back.shutdown(Shutdown::Both).unwrap();
        }
        self.back = None;
    }

    pub fn do_tls_read(&mut self) {
        // Read some TLS data.
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            let err = rc.unwrap_err();

            if let io::ErrorKind::WouldBlock = err.kind() {
                return;
            }

            println!("read error {:?}", err);
            self.closing = true;
            return;
        }

        if rc.unwrap() == 0 {
            println!("eof");
            self.closing = true;
            return;
        }

        // Process newly-received TLS messages.
        let processed = self.tls_session.process_new_packets();
        if processed.is_err() {
            println!("cannot process packet: {:?}", processed);
            self.closing = true;
            return;
        }
    }

    pub fn try_plain_read(&mut self) {
        // Read and process all available plaintext.
        let mut buf = Vec::new();

        let rc = self.tls_session.read_to_end(&mut buf);
        // println!("[MESSAGE] {}", String::from_utf8_lossy(&buf).into_owned());
        if rc.is_err() {
            println!("plaintext read failed: {:?}", rc);
            self.closing = true;
            return;
        }

        if !buf.is_empty() {
            self.incoming(&buf);
        }
    }

    pub fn try_back_read(&mut self) {
        if self.back.is_none() {
            return;
        }

        // Try a non-blocking read.
        let mut buf = [0u8; 1024];
        let back = self.back.as_mut().unwrap();
        let rc = try_read(back.read(&mut buf));

        if rc.is_err() {
            println!("backend read failed: {:?}", rc);
            self.closing = true;
            return;
        }

        let maybe_len = rc.unwrap();

        // If we have a successful but empty read, that's an EOF.
        // Otherwise, we shove the data into the TLS session.
        match maybe_len {
            Some(len) if len == 0 => {
                println!("back eof");
                self.closing = true;
            }
            Some(len) => {
                self.tls_session.write_all(&buf[..len]).unwrap();
            }
            None => {}
        };
    }

    fn send_settings(&mut self) {
        let head = Head {
            length: 0,
            kind: 4,
            flags: 0,
            stream_id: 0, 
        };
        self.tls_session
            .write_all(&head.as_bytes())
            .unwrap();
    } 

    fn send_settings_a(&mut self) {
        let head = Head {
            length: 0,
            kind: 4,
            flags: 1,
            stream_id: 0, 
        };
        self.tls_session
            .write_all(&head.as_bytes())
            .unwrap();
    } 

    fn send_response(&mut self) {
        let mut headers = Headers::new(7);
        headers.insert(String::from(":status"), String::from("200"));
        headers.insert(String::from("content-length"), String::from("13"));
        println!("{:?}", headers.as_bytes());
        let mut data = headers.as_bytes();
        let mut d = Data::new();
        data.append(&mut d.as_bytes());
        self.tls_session.write_all(&data.clone());
    }

    fn print_result(&self, frames: Vec<Frame>) {
        println!("-- [RESULT] ✅ ----------------");
        let mut unknown = 0;
        for frame in frames {
            match frame {
                Frame::Unknown(frame) => {
                    println!("[Unknown]");
                },
                _ => {
                    println!("{:?}", frame);
                }

            };
            println!("------------------------------");
            
        }

    }

    /// Process some amount of received plaintext.
    pub fn incoming(&mut self, buf: &[u8]) {
        if self.h2_session.is_accepted() {
            let frames = codec::parse_frames_from_buffer(&buf);
            self.print_result(frames);
            self.send_settings_a();
            self.send_response();
        } else {
            if proto::handshake(buf) {  
                self.h2_session.accept();
                let frames = codec::parse_frames_from_buffer(&buf[24..]);
                self.print_result(frames);
                self.send_settings();
                self.send_settings_a();
            }
        }

        
    }

    // pub fn send_http_response_once(&mut self) {
    //     let response = b"HTTP/1.0 200 OK\r\nConnection: close\r\n\r\nHello world from rustls tlsserver\r\n";
    //     self.tls_session
    //         .write_all(response)
    //         .unwrap();
    //     self.tls_session.send_close_notify();
    // }

    pub fn do_tls_write(&mut self) {
        let rc = self.tls_session.write_tls(&mut self.socket);
        if rc.is_err() {
            println!("write failed {:?}", rc);
            self.closing = true;
            return;
        }
    }

    pub fn register(&self, poll: &mut mio::Poll) {
        poll.register(&self.socket,
                      self.token,
                      self.event_set(),
                      mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();

        if self.back.is_some() {
            poll.register(self.back.as_ref().unwrap(),
                          self.token,
                          mio::Ready::readable(),
                          mio::PollOpt::level() | mio::PollOpt::oneshot())
                .unwrap();
        }
    }

    pub fn reregister(&self, poll: &mut mio::Poll) {
        poll.reregister(&self.socket,
                        self.token,
                        self.event_set(),
                        mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();

        if self.back.is_some() {
            poll.reregister(self.back.as_ref().unwrap(),
                            self.token,
                            mio::Ready::readable(),
                            mio::PollOpt::level() | mio::PollOpt::oneshot())
                .unwrap();
        }
    }

    /// What IO events we're currently waiting for,
    /// based on wants_read/wants_write.
    pub fn event_set(&self) -> mio::Ready {
        let rd = self.tls_session.wants_read();
        let wr = self.tls_session.wants_write();

        if rd && wr {
            mio::Ready::readable() | mio::Ready::writable()
        } else if wr {
            mio::Ready::writable()
        } else {
            mio::Ready::readable()
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}