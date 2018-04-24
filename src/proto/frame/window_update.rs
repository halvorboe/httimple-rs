use proto::frame::head::Head;

use bitreader::BitReader;

#[derive(Debug)]
pub struct WindowUpdate {
    head: Head, 
    window_size_increment: u32
}

impl WindowUpdate {

    pub fn from(head: Head, buf: Vec<u8>) -> WindowUpdate {
        let mut reader = BitReader::new(&buf);
        // Check that data is the rigth format
        println!("[PRIORITY] Stated reading settings...");
        reader.skip(1).unwrap();
        let window_size_increment = reader.read_u32(31).unwrap();
        println!("[PRIORITY] Read settings 😊");
        WindowUpdate { head: head, window_size_increment: window_size_increment }
    }

}