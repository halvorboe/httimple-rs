use std::collections::HashMap;
use hpack_codec::{Decoder, Encoder};
use hpack_codec::table::{StaticEntry};
use proto::frame::HEADERS;
use proto::frame::head::Head;



#[derive(Debug)]
pub struct Headers {
    head: Head,
    inner: HashMap<Vec<u8>, Vec<u8>>
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

        let inner = parse_header_block_fragment(&buf[cursor..]);

        Headers { head: head, inner: inner}
    }

    pub fn insert(&mut self, name: Vec<u8>, value: Vec<u8>) {
        self.inner.insert(name, value);
    }

    // Generates settings frame with default settings and
    pub fn new(stream_id: u32) -> Headers {
        let data = HashMap::new();
        Headers { head: Head { length: 0, kind: HEADERS, flags: 0, stream_id: stream_id }, inner: data }
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {
        let mut data = create_header_block_fragment(self.inner.clone()); 
        println!("{:?}", parse_header_block_fragment(&data));
        self.head.set_length(data.len() as u32);
        //self.head.set_flag(0);
        self.head.set_flag(2);
        let mut head = self.head.as_bytes();
        head.append(&mut data);
        println!("{:?}", self);
        head
    }

    pub fn is_end_stream(&self) -> bool {
        self.head.has_flag(0)
    } 

    pub fn is_end_headers(&self) -> bool {
        self.head.has_flag(2)
    } 

    pub fn get_headers(&self) -> HashMap<Vec<u8>, Vec<u8>> {
        self.inner.clone()
    } 
}

pub fn parse_header_block_fragment(buf: &[u8]) -> HashMap<Vec<u8>, Vec<u8>> {

    let mut inner = HashMap::new();

    let mut decoder = Decoder::new(4096); // Should not be a new decoder

    let mut header = decoder.enter_header_block(&buf[..]).unwrap();

    loop {
        match header.decode_field() {
            Ok(option) => {
                match option {
                    Some(field) => {
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

    inner
} 


pub fn create_header_block_fragment(headers: HashMap<Vec<u8>, Vec<u8>>) -> Vec<u8> {

    println!("{:?}", headers);

    let mut encoder = Encoder::new(4096); // Should not be a new encoder 

    let mut header = encoder.enter_header_block(Vec::new()).unwrap();
    header.encode_field(StaticEntry::Status200).unwrap(); // ContentLength

    header.finish()
}

// Write test to check that this works
// use hpack_codec::{Encoder, Decoder};

#[test] 

fn do_hpack() {
    // Encoding
    let mut encoder = Encoder::new(4096);
    let mut header = encoder.enter_header_block(Vec::new()).unwrap();
    header.encode_field(StaticEntry::MethodGet).unwrap();
    header.encode_field(Field::with_indexed_name(StaticEntry::Path, b"/hello")).unwrap();
    header.encode_field(Field::new(b"foo", b"bar").with_indexing()).unwrap();
    header.encode_field(Index::dynamic_table_offset() + 0).unwrap();
    let encoded_data = header.finish();

    // Decoding
    let mut decoder = Decoder::new(4096);
    let mut header = decoder.enter_header_block(&encoded_data[..]).unwrap();
    assert_eq!(header.decode_field().unwrap(), HeaderField::new(b":method", b"GET").ok());
    assert_eq!(header.decode_field().unwrap(), HeaderField::new(b":path", b"/hello").ok());
    assert_eq!(header.decode_field().unwrap(), HeaderField::new(b"foo", b"bar").ok());
    assert_eq!(header.decode_field().unwrap(), HeaderField::new(b"foo", b"bar").ok());
}




    // header.encode_field(Field::with_indexed_name(StaticEntry::ContentLength, b"22")).unwrap(); // ContentLength
    // header.encode_field(Field::with_indexed_name(StaticEntry::, b"/hello")).unwrap();
    // header.encode_field(Field::new(b"foo", b"bar").with_indexing()).unwrap();
    // header.encode_field(Index::dynamic_table_offset() + 0).unwrap();
    //     for (name, value) in &headers {
//         temp.push((name.as_bytes(), value.as_bytes()).clone());
//     }
//     let mut encoder = Encoder::new();
//     let e = encoder.encode(temp);
//     println!("e -----> {:?}", e);
//     e