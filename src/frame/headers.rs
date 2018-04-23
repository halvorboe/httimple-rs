use hpack::Decoder;

use frame::HEADERS;
use frame::head::Head;

#[derive(Debug)]
pub struct Headers {
    head: Head,
    inner: String
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
        let mut decoder = Decoder::new();

        decoder.decode_with_cb(&buf[cursor..], |name, value| {
            let n = String::from_utf8(name.to_vec()).unwrap();
            let v = String::from_utf8(value.to_vec()).unwrap();
            println!("{:?} -> {:?}", n, v);
        });

        // let data = String::from_utf8(buf).unwrap(); 
        Headers { head: head, inner: String::from("")}
    }

    // Generates settings frame with default settings and
    pub fn new(stream_id: u32) -> Headers {
        let data = String::from("");
        Headers { head: Head { length: data.len() as u32, kind: HEADERS, flags: 0, stream_id: stream_id }, inner: data }
    }
}

