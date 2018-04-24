pub mod codec;
pub mod frame; 
pub mod session;
pub mod util;

const PREFACE : &str = "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";


pub fn handshake(buf: &[u8]) -> bool {
    let sent = String::from_utf8(buf[0..24].to_vec());
    match sent {
        Ok(msg) => {
            if msg == PREFACE {
                println!("[HANDSHAKE] Handshake accepted 😊");
                true
            } else {
                println!("[HANDSHAKE] Handshake rejected 💔");
                false
            }
        },
        _ => false
    }

}


