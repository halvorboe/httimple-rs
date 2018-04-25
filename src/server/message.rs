use proto::frame::headers::Headers;
use proto::frame::data::Data;

use rustls::ServerSession;
use std::io::Write;


#[derive(Debug)]
pub struct Message {
    pub status: u32,
}

impl Message {

    pub fn send(&self, session: &mut ServerSession) {
        println!("[SENDING] Status: {}", self.status);
        if self.status == 200 {
            let mut headers = Headers::new(1);
            session.write_all(&headers.as_bytes()).unwrap();
            let mut d = Data::new(1);
            session.write_all(&d.as_bytes()).unwrap();
        }
    } 

    pub fn not_found() -> Message {
        Message { status: 404 }
    }

}