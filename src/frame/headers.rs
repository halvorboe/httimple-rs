use hpack::{Decoder, Encoder};
use std::collections::HashMap;
use util;
use frame::HEADERS;
use frame::head::Head;

#[derive(Debug)]
pub struct Headers {
    head: Head,
    inner: HashMap<String, String>
}

impl Headers {
    pub fn from(head: Head, buf: Vec<u8>) -> Headers {
        let mut cursor = 0;
        // Flag 3 = Pad 
        if head.has_flag(3) {
            cursor += 1;
        }
        // Flag 5 = Stream info 
        if head.has_flag(5) {
            cursor += 5;
        }

        let inner = util::parse_header_block_fragment(&buf[cursor..]);

        Headers { head: head, inner: inner}
    }

    pub fn insert(&mut self, name: String, value: String) {
        self.inner.insert(name, value);
    }

    // Generates settings frame with default settings and
    pub fn new(stream_id: u32) -> Headers {
        let data = HashMap::new();
        Headers { head: Head { length: 0, kind: HEADERS, flags: 0, stream_id: stream_id }, inner: data }
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {
        let mut data = util::create_header_block_fragment(self.inner.clone()); 
        self.head.set_length(data.len() as u32);
        self.head.set_flag(2);
        let mut head = self.head.as_bytes();
        head.append(&mut data);
        println!("{:?}", self);
        head
    }
}

