

use frame::head::Head;
use bitreader::BitReader;

#[derive(Debug)]
pub struct Data {
    head: Head,
    inner: Vec<u8>,
}

impl Data {

    pub fn from(head: Head, buf: Vec<u8>) -> Data {
        println!("[DATA] Stated reading data...");
        let data = Vec::new();
        if head.has_flag(3) {   
            let mut reader = BitReader::new(&buf);
            let buffer = reader.read_u32(8).unwrap();
            let to = (head.length - buffer) as usize;
            let data = buf[1..to].to_vec();
        } else {
            let data = buf;
        }
        println!("[DATA] Read data 😊");
        Data { head: head, inner: data }
    }

    pub fn new() -> Data {
        Data { 
            head: Head { length: 0, kind: 0, flags: 0, stream_id: 7
            },
            inner: {
                vec![24, 23, 23, 23, 23, 26, 45, 45, 23, 12, 11, 12, 23]
            }
        }
    } 

    pub fn as_bytes(&mut self) -> Vec<u8> {
        self.head.set_length(self.inner.len() as u32);
        self.head.set_flag(0);
        let mut head = self.head.as_bytes();
        println!("{:?}", self.inner);
        head.append(&mut self.inner);
        println!("{:?}", self);
        println!("{:?}", head);
        head
    }
    

}