use proto::frame::headers::Headers;
use proto::frame::data::Data;

use rustls::ServerSession;
use std::io::Write;


#[derive(Debug)]
pub struct Message {
    pub status: u32,
}

impl Message {

    pub fn send(&self, session: &mut ServerSession, stream_id: &mut u32) {
        if *stream_id == 0 {
            *stream_id += 1;
        }
        println!("[SENDING] Status: {} Stream: {}", self.status, stream_id);
        if self.status == 200 {
            let mut headers = Headers::new(*stream_id);
            session.write_all(&headers.as_bytes()).unwrap();
            let mut d = Data::new(*stream_id);
            session.write_all(&d.as_bytes()).unwrap();
        }
    } 

    pub fn not_found() -> Message {
        Message { status: 404 }
    }

}