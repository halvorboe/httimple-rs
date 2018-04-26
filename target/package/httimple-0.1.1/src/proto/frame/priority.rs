
use proto::frame::head::Head;
use bitreader::BitReader;

#[derive(Debug)]
pub struct Priority {
    head: Head,
    stream_dependency: u32,
    weigth: u8
}

impl Priority {

    pub fn from(head: Head, buf: Vec<u8>) -> Priority {
        let mut reader = BitReader::new(&buf);
        // Check that data is the rigth format
        println!("[PRIORITY] Stated reading priority...");
        reader.skip(1).unwrap();
        let stream_dependency = reader.read_u32(31).unwrap();
        let weigth = reader.read_u8(8).unwrap();
        println!("[PRIORITY] Read priority 😊");
        Priority { head: head, stream_dependency: stream_dependency, weigth: weigth }
    }

}