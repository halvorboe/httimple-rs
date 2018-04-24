
use bitreader::BitReader;
use bytes::Bytes; 
use proto::util;

// Map number to type

#[derive(Debug)]
pub struct Head {
    pub length: u32,
    pub kind: u8,
    pub flags: u8,
    pub stream_id: u32
}

impl Head {
    pub fn as_bytes(&self) -> Vec<u8> {
        vec![
            (((self.length >> 16) & 0x000000FF) as u8),
            (((self.length >>  8) & 0x000000FF) as u8),
            (((self.length      ) & 0x000000FF) as u8),
            self.kind,
            self.flags,
            (((self.stream_id >> 24) & 0x000000FF) as u8),
            (((self.stream_id >> 16) & 0x000000FF) as u8),
            (((self.stream_id >>  8) & 0x000000FF) as u8),
            (((self.stream_id ) & 0x000000FF) as u8)
        ]
    }

    pub fn clone(&self) -> Head {
        Head { length: self.length, kind: self.kind, flags: self.flags, stream_id: self.stream_id }
    } 

    pub fn has_flag(&self, index: u8) -> bool {
        if index < 8 {
            let filter = (1 << index);
            (filter  & self.flags) != 0
        } else {
            false
        }
    } 

    pub fn set_flag(&mut self, index: u8) {
        self.flags += 1 << index;
    } 

    pub fn set_length(&mut self, length: u32) {
        self.length = length;
    }

}

#[test] 
fn head_has_flag() {
    let head = Head { length: 0, kind: 0, flags: 1, stream_id: 0 };
    assert_eq!(head.has_flag(0), true);
    assert_eq!(head.has_flag(1), false);
    let head = Head { length: 0, kind: 0, flags: 2, stream_id: 0 };
    assert_eq!(head.has_flag(1), true);
    assert_eq!(head.has_flag(2), false);
    let head = Head { length: 0, kind: 0, flags: 4, stream_id: 0 };
    assert_eq!(head.has_flag(2), true);
    assert_eq!(head.has_flag(3), false);
    let head = Head { length: 0, kind: 0, flags: 128, stream_id: 0 };
    assert_eq!(head.has_flag(7), true);
    assert_eq!(head.has_flag(6), false);
}

#[test]
fn head_to_bytes() {
    let head = Head { length: 0, kind: 0, flags: 0, stream_id: 0 };
    assert_eq!(9, head.as_bytes().len());
}

#[test] 
fn head_set_flag() {
    let mut head = Head { length: 0, kind: 0, flags: 0, stream_id: 0 };
    head.set_flag(0);
    assert_eq!(head.has_flag(0), true);
    assert_eq!(head.has_flag(1), false);
}

impl From<Vec<u8>> for Head {

    // This should be a result

    fn from(buf : Vec<u8>) -> Head {
        if buf.len() >= 9 {
            let t = buf[0..9].to_vec();
            let h = t.as_slice();
            let mut reader = BitReader::new(h);
            let length = reader.read_u32(24).unwrap();
            println!("[OK] Length: {}", length);
            let kind = reader.read_u8(8).unwrap();
            println!("[OK] Kind: {} ({})", kind, util::get_type(kind));
            let flags = reader.read_u8(8).unwrap();
            println!("[OK] Flags: {:b}", flags);
            let stream_id = reader.read_u32(31).unwrap();
            println!("[OK] Stream id: {}", stream_id);
            Head { length: length, kind: kind, flags: flags, stream_id: stream_id }
        } else {
            println!("[ERROR] Not enougth bytes to build header: {}", buf.len());
            Head { length: 0, kind: 0, flags: 0, stream_id: 0 }
        }
       
    }
}

