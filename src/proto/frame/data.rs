

use proto::frame::head::Head;
use bitreader::BitReader;

// Debug only 
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct Data {
    head: Head,
    inner: Vec<u8>,
}

impl Data {

    pub fn from(head: Head, buf: Vec<u8>) -> Data {
        println!("[DATA] Stated reading data...");
        let mut data = Vec::new();
        if head.has_flag(3) {   
            let mut reader = BitReader::new(&buf);
            let buffer = reader.read_u32(8).unwrap();
            let to = (head.length - buffer) as usize;
            let mut a = buf[1..to].to_vec().clone();
            data.append(&mut a);
        } 
        println!("[DATA] Read data 😊");
        Data { head: head, inner: data }
    }

    pub fn new(stream_id: u32) -> Data {
        let mut f = File::open("index.html").unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        Data { 
            head: Head { length: 0, kind: 0, flags: 0, stream_id: stream_id
            },
            inner: {
                buffer.as_bytes().to_vec()
            }
        }
    } 

    pub fn as_bytes(&mut self) -> Vec<u8> {
        self.head.set_length(self.inner.len() as u32);
        self.head.set_flag(0);
        let mut head = self.head.as_bytes();
        //println!("{:?}", self.inner);
        head.append(&mut self.inner);
        println!("{:?}", self);
        //println!("{:?}", head);
        head
    }

    pub fn get_payload(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub fn is_end_stream(&self) -> bool {
        self.head.has_flag(0)
    } 
    

}