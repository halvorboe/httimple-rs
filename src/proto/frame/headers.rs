use std::collections::HashMap;
use hpack_codec::{Decoder, Encoder};
use hpack_codec::table::{StaticEntry};
use proto::frame::HEADERS;
use proto::frame::head::Head;

#[derive(Debug, Clone)]
enum HeaderBlock {
    Raw(Vec<u8>),
    Uncompressed(HashMap<Vec<u8>, Vec<u8>>)
}


#[derive(Debug)]
pub struct Headers {
    head: Head,
    inner: HeaderBlock,
}

#[allow(dead_code)]
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

        Headers { head: head, inner: HeaderBlock::Raw(buf[cursor..].to_vec())}
    }

    pub fn insert(&mut self, name: Vec<u8>, value: Vec<u8>) {
        match self.inner {
            HeaderBlock::Raw(_) => {
                println!("[Error] Tried to insert into compressed header")
            },
            HeaderBlock::Uncompressed(ref mut headers) => {
                headers.insert(name, value);
            }
        };
    }

    // Decodes the raw data into a HashMap

    pub fn decode(&mut self, decoder: &mut Decoder) {
        match self.inner.clone() {
            HeaderBlock::Raw(ref mut buf) => {
                let mut inner = HashMap::new();
                let mut header = decoder.enter_header_block(&buf[..]).unwrap();

                loop {
                    match header.decode_field() {
                        Ok(option) => {
                            match option {
                                Some(ref field) => {
                                    inner.insert(field.name().to_vec(), field.value().to_vec());
                                },
                                _ => {
                                    break;
                                }
                            }
                        },
                        Err(err) => {
                            println!("[ERROR] {:?}", err);
                        }
                    }

                }

                self.inner = HeaderBlock::Uncompressed(inner);
            },
            _ => {}
        }
    }

    pub fn encode(&mut self, encoder: &mut Encoder) -> Vec<u8> {
        let mut headers = encoder.enter_header_block(Vec::new()).unwrap();
        headers.encode_field(StaticEntry::Status200).unwrap(); // ContentLength
        println!("Encoding...");
        let mut data = headers.finish();
        self.head.set_length(data.len() as u32);
        let mut head = self.head.as_bytes();
        head.append(&mut data);
        println!("{:?}", self);
        head
    }

    // Generates settings frame with default settings and
    pub fn new(stream_id: u32) -> Headers {
        let data = HashMap::new();
        Headers { head: Head { length: 0, kind: HEADERS, flags: 0, stream_id: stream_id }, inner: HeaderBlock::Uncompressed(data)}
    }

    pub fn is_end_stream(&self) -> bool {
        self.head.has_flag(0)
    } 

    pub fn is_end_headers(&self) -> bool {
        self.head.has_flag(2)
    } 

    pub fn end_stream(&mut self) {
        self.head.set_flag(0);
    }

    pub fn end_headers(&mut self) {
        self.head.set_flag(2);
    }

    pub fn get_headers(&self) -> Option<HashMap<Vec<u8>, Vec<u8>>> {
        match self.inner {
            HeaderBlock::Uncompressed(ref headers) => {
                Some(headers.clone())
            },
            _ => {
                None
            }
        }
        
    } 
}
