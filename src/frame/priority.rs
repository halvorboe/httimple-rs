
use frame::head::Head;


pub struct Priority {
    head: Head;
    stream_dependency: u32,
    weigth: u8
}

impl Priority {

    pub fn from(head: Head, buf: Vec<u8>) -> Priority {
        let mut reader = BitReader::new(&buf);
        // Check that data is the rigth format
        println!("[PRIORITY] Stated reading settings...");
        reader.skip(1);
        let stream_dependency = reader.read_u32(31).unwrap();
        let weigth = reader.read_u8(8).unwrap();
        println!("[PRIORITY] Read settings 😊");
        Priority { head: head, stream_dependency: stream_dependency, weigth: weigth }
    }

}