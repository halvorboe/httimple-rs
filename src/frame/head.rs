
use bitreader::BitReader;
use bytes::Bytes; 
use frame::util;

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
    }

    pub fn clone(&self) -> Head {
        Head { length: self.length, kind: self.kind, flags: self.flags, stream_id: self.stream_id }
    } 

    pub fn has_flag(&self, index: u8) -> bool {
        if index < 8 {
            let filter = (1 << (7 - index));
            (filter  & self.flags) != 0
        } else {
            false
        }
    } 

}

#[test] 
fn head_has_flag() {
    let head = Head { length: 0, kind: 0, flags: 128, stream_id: 0 };
    assert_eq!(head.has_flag(0), true);
    assert_eq!(head.has_flag(1), false);
    let head = Head { length: 0, kind: 0, flags: 64, stream_id: 0 };
    assert_eq!(head.has_flag(1), true);
    assert_eq!(head.has_flag(2), false);
    let head = Head { length: 0, kind: 0, flags: 32, stream_id: 0 };
    assert_eq!(head.has_flag(2), true);
    assert_eq!(head.has_flag(3), false);
    let head = Head { length: 0, kind: 0, flags: 1, stream_id: 0 };
    assert_eq!(head.has_flag(7), true);
    assert_eq!(head.has_flag(6), false);
}

#[test]
fn head_to_bytes() {
    let head = Head { length: 0, kind: 0, flags: 0, stream_id: 0 };
    assert_eq!(9, head.as_bytes().len());
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

