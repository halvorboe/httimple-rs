mod settings;
mod head; 
mod util; 


use self::head::Head;

use bytes::Bytes;
use self::settings::Settings;

pub type Size = u32;

pub enum Frame {
    Raw(Vec<u8>),
    Settings(Settings),
}

impl From<Bytes> for Frame {
    fn from(buf: Bytes) -> Frame {
        let head = Head::from(buf.clone());
        println!("{}", head.as_bytes().len());
        if head.length / 8 == buf.len() as u32 - 9 {
            println!("Propper frame of length: {}", head.length);
        }
        Frame::Raw(buf.to_vec())
    }
}