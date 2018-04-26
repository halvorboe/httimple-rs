pub mod settings;
pub mod head; 
pub mod headers;
pub mod priority;
pub mod continuation;
pub mod window_update;
pub mod data;

use self::head::Head;
use self::data::Data;
use self::settings::Settings;
use self::headers::Headers;
use self::priority::Priority;
use self::continuation::Continuation;
use self::window_update::WindowUpdate;

pub const DATA : u8 = 0;
pub const HEADERS : u8 = 1;
pub const PRIORITY : u8 = 2;
pub const RST_STREAM : u8 = 3;
pub const SETTINGS : u8 = 4;
pub const PUSH_PROMISE : u8 = 5;
pub const PING : u8 = 6;
pub const GOAWAY : u8 = 7;
pub const WINDOW_UPDATE : u8 = 8;
pub const CONTINUATION : u8 = 9;

#[derive(Debug)]
pub enum Frame {
    Data(Data),
    Headers(Headers),
    Priority(Priority),
    Settings(Settings),
    WindowUpdate(WindowUpdate),
    Continuation(Continuation),
    Unknown(Vec<u8>), 
}

impl Frame {
    pub fn parse(buf: &[u8]) -> (Frame, usize, u32) {
        println!("[FRAME] Parsing frame from {} bytes.", buf.len());
        let head = head::Head::from(buf.to_vec());
        let data = &buf[9..9 + head.length as usize];
        let frame = {
            match head.kind {
                DATA => Frame::Data(Data::from(head.clone(), data.to_vec())),
                HEADERS => Frame::Headers(Headers::from(head.clone(), data.to_vec())),
                PRIORITY => Frame::Priority(Priority::from(head.clone(), data.to_vec())),
                SETTINGS => Frame::Settings(Settings::from(head.clone(), data.to_vec())),
                WINDOW_UPDATE => Frame::WindowUpdate(WindowUpdate::from(head.clone(), data.to_vec())),
                CONTINUATION => Frame::Continuation(Continuation::from(head.clone(), data.to_vec())),
                _ => Frame::Unknown(data.to_vec())
            }
        };
        (frame, 9 + head.length as usize, head.stream_id)
    }

    pub fn is_call(&self) -> bool {
        let frame = self;
        match frame {
            &Frame::Data(_) | &Frame::Headers(_) | &Frame::Continuation(_) => {
                true
            },
            _ => {
                false
            }
        }
    }
}