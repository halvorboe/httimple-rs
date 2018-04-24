use proto::frame::{DATA, HEADERS, PRIORITY, RST_STREAM, SETTINGS, PING, GOAWAY, WINDOW_UPDATE, CONTINUATION};
use std::collections::HashMap;
use hpack_codec::{Decoder, Encoder};
use hpack_codec::field::{HeaderField, LiteralHeaderField as Field};
use hpack_codec::table::{StaticEntry, Index};


pub fn pow(n: u32, t: u32) -> u32 {
    if t == 0 {
        1
    } else {
        let mut g = n;
        for _ in 0..t - 1 {
            g *= n;
        }
        g
    }
}

#[test]
fn do_pow() {
    assert_eq!(1, pow(4, 0));
    assert_eq!(4, pow(4, 1));
    assert_eq!(16, pow(4, 2));
    assert_eq!(64, pow(4, 3));
}

const PAD : &str = "0";

pub fn pad(s : String, len : usize) -> String{
    if s.len() < len {
        let i = len - s.len();
        let mut t = String::new();
        for _ in 0..i {
            t.push_str(PAD);
        }
        t.push_str(&s);
        t
    } else {
        s
    }
} 

#[test]
fn do_pad() {
    let mut s = String::from("1");
    let t = String::from("0001");
    assert_eq!(t, pad(s, 4));
    let mut s = String::from("1111");
    let t = String::from("1111");
    assert_eq!(t, pad(s, 4));
}

pub fn bin_to_vec(s : String) -> Vec<u8> {
    assert_eq!(0, s.len() % 8);
    let mut v = Vec::new();
    let len = s.len() / 8;
    println!("{}", len);
    let mut c = s.chars();
    for _ in 0..len {
        let mut i : u8 = 0;
        for y in 0..8 {
            if c.next().unwrap() == '1' {
                i += pow(2, 7 - y as u32) as u8;
            }
        }
        v.push(i);

    }
    v 
}


#[test]
fn do_bin_to_vec() {
    let t = String::from("00000001");
    let v = bin_to_vec(t);
    assert_eq!(1, v[0]);
    let t = String::from("11111111");
    let v = bin_to_vec(t);
    assert_eq!(255, v[0]);
}


pub fn get_type(i: u8) -> String {
   let r = {
       match i {
           DATA => "DATA",
           HEADERS => "HEADERS",
           PRIORITY => "PRIORITY",
           RST_STREAM => "RST_STREAM",
           SETTINGS => "SETTINGS",
           PING => "PING",
           GOAWAY => "GOAWAY",
           WINDOW_UPDATE => "WINDOW_UPDATE",
           CONTINUATION => "CONTINUATION",
           _ => "UNKNOWN"
       }
   };
   String::from(r)
}


pub fn parse_header_block_fragment(buf: &[u8]) -> HashMap<String, String> {
    // let mut decoder = Decoder::new();

    let mut inner = HashMap::new();

    // match decoder.decode_with_cb(buf, |name, value| {
    //     let n = String::from_utf8_lossy(&name).into_owned();
    //     let v = String::from_utf8_lossy(&value).into_owned();
    //     inner.insert(n, v);
    // }) {
    //     Err(err) => println!("{:?}", err),
    //     Ok(ok) => {}
    // }

    inner
} 


pub fn create_header_block_fragment(headers: HashMap<String, String>) -> Vec<u8> {
    let mut encoder = Encoder::new(4096);
    let mut header = encoder.enter_header_block(Vec::new()).unwrap();
    header.encode_field(StaticEntry::Status200).unwrap(); // ContentLength
    // header.encode_field(Field::with_indexed_name(StaticEntry::ContentLength, b"22")).unwrap(); // ContentLength
    // header.encode_field(Field::with_indexed_name(StaticEntry::, b"/hello")).unwrap();
    // header.encode_field(Field::new(b"foo", b"bar").with_indexing()).unwrap();
    // header.encode_field(Index::dynamic_table_offset() + 0).unwrap();
    header.finish()
//     for (name, value) in &headers {
//         temp.push((name.as_bytes(), value.as_bytes()).clone());
//     }
//     let mut encoder = Encoder::new();
//     let e = encoder.encode(temp);
//     println!("e -----> {:?}", e);
//     e
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