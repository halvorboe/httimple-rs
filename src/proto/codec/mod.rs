use std;
use bytes::{Bytes, BytesMut};
use proto::frame::Frame;
use std::io;
use proto::util;


pub fn parse_frames_from_buffer(buf: &[u8]) -> Vec<Frame> {
    let mut pointer = 0;
    let mut frames = Vec::new(); 
    println!("[PARSE STARTED]");
    while pointer < buf.len() {
        let b = buf[pointer..].to_vec();
        let (frame, length) = Frame::parse(b);
        frames.push(frame);
        let data = &buf[pointer..pointer + length];
        println!("-- [DATA] --------------------");
        println!("{}", String::from_utf8_lossy(data));
        println!("-- [RAW] ---------------------");
        for (n, i) in data.iter().enumerate() {
            print!("{}", util::pad(format!("{:b} ", i), 9));
            if n % 8 == 7 {
                print!("\n");
            }
        }
        print!("\n");
        println!("------------------------------");
        pointer += length
    }
    println!("[PARSE ENDED]");
    frames
}

