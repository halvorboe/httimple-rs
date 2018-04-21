
use bitreader::BitReader;
use bytes::Bytes; 
use frame::util;

// Map number to type

pub struct Head {
    pub length: u32,
    pub kind: u8,
    pub flags: u8,
    pub stream_id: u32
}

impl Head {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut string = String::from("");
        // Add length
        string.push_str(
            &util::pad(format!("{:b}", self.length), 24)
        );
        // Add kind
        string.push_str(
            &util::pad(format!("{:b}", self.kind), 8)
        );
        // Add kind
        string.push_str(
            &util::pad(format!("{:b}", self.flags), 8)
        );
        // Reserved bit
        string.push_str("0");
        // Add kind        
        string.push_str(
            &util::pad(format!("{:b}", self.stream_id), 31)
        );
        println!("L: {} R: {}", string.len(), string);
        util::bin_to_vec(string) 
        // let h = Vec::new();
        // h
    }
}

#[test]
fn head_to_bytes() {
    let head = Head { length: 0, kind: 0, flags: 0, stream_id: 0 };
    assert_eq!(9, head.as_bytes().len());
}

impl From<Bytes> for Head {
    fn from(buf : Bytes) -> Head {
        if buf.len() > 9 {
            let t = buf[0..9].to_vec();
            let h = t.as_slice();
            let mut reader = BitReader::new(h);
            let length = reader.read_u32(24).unwrap();
            println!("[OK] Length: {:b}", length);
            let kind = reader.read_u8(8).unwrap();
            println!("[OK] Kind: {:b}", kind);
            let flags = reader.read_u8(8).unwrap();
            println!("[OK] Flags: {:b}", flags);
            let stream_id = reader.read_u32(31).unwrap();
            println!("[OK] Stream id: {:b}", stream_id);
            Head { length: length, kind: kind, flags: flags, stream_id: stream_id }
        } else {
            println!("[ERROR] Not enougth bytes to build header: {}", buf.len());
            Head { length: 0, kind: 0, flags: 0, stream_id: 0 }
        }
       
    }
}

/*

24 bits for lenght 
8 for type 
*/