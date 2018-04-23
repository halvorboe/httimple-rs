pub mod settings;
pub mod head; 
pub mod util; 
pub mod headers;

use self::head::Head;

use self::settings::Settings;
use self::headers::Headers;

pub type Size = u32;

pub const DATA : u8 = 0;
pub const HEADERS : u8 = 1;
pub const PRIORITY : u8 = 2;
pub const RST_STREAM : u8 = 3;
pub const SETTINGS : u8 = 4;
pub const PING : u8 = 5;
pub const GOAWAY : u8 = 6;
pub const WINDOW_UPDATE : u8 = 7;
pub const CONTINUATION : u8 = 8;

#[derive(Debug)]
pub enum Frame {
    Headers(Headers),
    Settings(Settings),
    Unknown(Vec<u8>), 
}

impl Frame {
    pub fn parse(buf: Vec<u8>) -> (Frame, usize) {
        println!("[FRAME] Parsing frame from {} bytes.", buf.len());
        let head = head::Head::from(buf.clone());
        let data = &buf[9..9 + head.length as usize];
        let frame = {
            match head.kind {
                1 => Frame::Headers(Headers::from(head.clone(), data.to_vec())),
                4 => Frame::Settings(Settings::from(head.clone(), data.to_vec())),
                _ => Frame::Unknown(data.to_vec())
            }
        };
        (frame, 9 + head.length as usize)
    }
}