use frame::{SETTINGS, Size};
use frame::Head;
use bitreader::BitReader;


pub const DEFAULT_SETTINGS_HEADER_TABLE_SIZE: Size = 4_096;
pub const DEFAULT_INITIAL_WINDOW_SIZE: Size = 65_535;
pub const DEFAULT_MAX_FRAME_SIZE: Size = 16_384;
pub const MAX_INITIAL_WINDOW_SIZE: Size = (1 << 31) - 1;
pub const MAX_MAX_FRAME_SIZE: Size = (1 << 24) - 1;

#[derive(Debug)]
pub struct Setting {
    id: u16,
    value: u32,
}

#[derive(Debug)]
pub struct Settings {
    head: Head,
    inner: Vec<Setting>
}

impl Settings {
    
    pub fn from(head: Head, buf: Vec<u8>) -> Settings {
        let mut settings = Vec::new();
        let mut reader = BitReader::new(&buf);
        // Check that data is the rigth format
        println!("[SETTINGS] Stated reading settings...");
        if buf.len() % 3 == 0 {
            while reader.position() < buf.len() as u64 {
                let id = reader.read_u16(16).unwrap();
                let value = reader.read_u32(32).unwrap();
                settings.push(Setting {id: id, value: value});
                println!("{} -> {}", id, value);
            }
            println!("[SETTINGS] Read settings 😊");
        } else {
            println!("[SETTINGS] Failed reading settings 😢");
        }
        Settings { head: head, inner: settings}
    }

    // Generates settings frame with default settings and
    pub fn new(stream_id: u32) -> Settings {
        let mut settings = Vec::new();
        settings.push(Setting { id: 1, value: DEFAULT_SETTINGS_HEADER_TABLE_SIZE });
        settings.push(Setting { id: 4, value: DEFAULT_INITIAL_WINDOW_SIZE });
        settings.push(Setting { id: 5, value: DEFAULT_MAX_FRAME_SIZE });
        Settings { head: Head { length: settings.len() as u32 * 3, kind: SETTINGS, flags: 0, stream_id: stream_id }, inner: settings}
    }

}