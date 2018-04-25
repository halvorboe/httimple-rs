use mio::tcp::{TcpStream, Shutdown};
use rustls;
use rustls::Session;
use mio;
use std::io;
use app::message::Message;
use std::io::{Read, Write};
use std::collections::HashMap;
use proto::session;
use proto;
use proto::frame::Frame;

use app::call::Call;

use proto::frame::head::Head;

use app::Callback;

pub struct Connection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    h2_session: session::Session,
    tls_session: rustls::ServerSession,
    calls: HashMap<u32, Call>,
    handler: HashMap<Vec<u8>, Callback>
}

impl Connection {

    pub fn new(socket: TcpStream, token: mio::Token, tls_session: rustls::ServerSession, handler: HashMap<Vec<u8>, Callback>)-> Connection {
        Connection {
            socket: socket,
            token: token,
            closing: false,
            closed: false,
            h2_session: session::Session { 
                accepted: false, 
                settings: HashMap::new(), 
                streams: HashMap::new() 
            },
            tls_session: tls_session,
            calls: HashMap::new(),
            handler: handler
        }
    }

    pub fn incoming(&mut self, buf: &[u8]) {
        if !self.h2_session.is_accepted() {
            if proto::handshake(buf) { 
                self.h2_session.accept();
                self.send_settings();
                self.handle(&buf[24..])
            } else {
                return;
            }
        } else {
            self.handle(buf);
        }
        
    }

    pub fn handle(&mut self, buf: &[u8]) {
        // let frames = codec::parse_frames_from_buffer(&buf);
        // self.print_result(frames);
        let mut cursor = 0; 
        while cursor < buf.len() {
            let (frame, length, stream_id) = Frame::parse(&buf[cursor..]);
            cursor += length;
            if frame.is_call() {
                self.call(stream_id, frame); // Data, Headers, Caller
            } else {
                self.modify(stream_id, frame);
            }
        }
    }

    pub fn call(&mut self, stream_id: u32, frame: Frame) {
        let call = self.calls.entry(stream_id).or_insert(Call::from());
        match frame {
            Frame::Data(data) => {
                call.insert_data(data);
            },
            Frame::Headers(headers) => {
                call.insert_headers(headers);
            },
            Frame::Continuation(continuation) => {
                call.insert_continuation(continuation);
            },
            _ => {}
        }
        if call.is_ready() {
            match call.path() {
                Some(path) => {
                    println!("PATH!!! {} ------------------", String::from_utf8(path.clone()).unwrap());
                    for (key, _) in &self.handler {
                        println!("{:?}", String::from_utf8(key.clone()).unwrap());
                    }
                    let message = match self.handler.get(path) {
                        Some(callback) => callback(call),
                        _ => Message::not_found()
                    };
                    let mut id = stream_id;
                    message.send(&mut self.tls_session, &mut id);
                },
                _ => {
                     println!("NO PATH!!! ------------------");
                }
            }
        }
    } 

    pub fn modify(&mut self, stream_id: u32, frame: Frame) {
        match frame {
            Frame::Settings(settings) => {
                println!("{:?}", settings);
                self.send_settings_a(stream_id);
            },
            _ => {}
        }
    }

    pub fn ready(&mut self, poll: &mut mio::Poll, ev: &mio::Event) {
        if ev.readiness().is_readable() && !self.closing {
            self.read_tls();
            self.read();
        }

        if ev.readiness().is_writable() {
            self.write_tls();
        }

        if self.closing && !self.tls_session.wants_write() {
            let _ = self.socket.shutdown(Shutdown::Both);
            self.closed = true;
        } else {
            self.reregister(poll);
        }
    }

    pub fn read_tls(&mut self) {
        let rc = self.tls_session.read_tls(&mut self.socket);

        if rc.is_err() {
            let err = rc.unwrap_err();
            if let io::ErrorKind::WouldBlock = err.kind() {} 
            else {
                println!("[ERROR] Read error: {:?}", err);
                self.closing = true;
            }
            return;
        }

        if rc.unwrap() == 0 {
            println!("[ERROR] EOF");
            self.closing = true;
            return;
        }

        let processed = self.tls_session.process_new_packets();

        if processed.is_err() {
            println!("[ERROR] Cannot process packet: {:?}", processed);
            self.closing = true;
            return;
        }

    }

    pub fn read(&mut self) {
        let mut buf = Vec::new();

        if self.tls_session.read_to_end(&mut buf).is_err() {
            self.closing = true;
        }

        if !buf.is_empty() {
            self.incoming(&buf);
        }
    }

    pub fn write_tls(&mut self) {
        let rc = self.tls_session.write_tls(&mut self.socket);

        if rc.is_err() {
            // println!("[ERROR] Write failed {:?}", rc);
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
    }

    pub fn reregister(&self, poll: &mut mio::Poll) {
        poll.reregister(&self.socket,
                        self.token,
                        self.event_set(),
                        mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();
    }

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

    // TEMP

    fn send_settings(&mut self) {
        self.send_settings_frame(0, 0);
    } 

    fn send_settings_a(&mut self, stream_id: u32) {
        self.send_settings_frame(1, stream_id);
    } 

    fn send_settings_frame(&mut self, flag: u8, stream_id: u32) {
        let head = Head {
            length: 0,
            kind: 4,
            flags: flag,
            stream_id: stream_id, 
        };
        let w = self.tls_session
            .write_all(&head.as_bytes())
            .unwrap();
        println!("SENT SETTINGS {:?}", w);
    }


    // fn send_response(tls_session: &mut rustls::ServerSession) {
    //     let mut headers = Headers::new(1);
    //     tls_session.write_all(&headers.as_bytes());
    //     let mut d = Data::new(1);
    //     tls_session.write_all(&d.as_bytes());
    //     // self.closing = true;
    // }

    // fn print_result(&self, frames: Vec<Frame>) {
    //     println!("-- [RESULT] ✅ ----------------");
    //     let mut unknown = 0;
    //     for frame in frames {
    //         match frame {
    //             Frame::Unknown(frame) => {
    //                 println!("[Unknown]");
    //             },
    //             _ => {
    //                 println!("{:?}", frame);
    //             }

    //         };
    //         println!("------------------------------");
            
    //     }

    // }

}