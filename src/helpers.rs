

// Functions to make life EZ
use std::io::prelude::*;
use std::fs::File;


pub fn file(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer.as_bytes().to_vec()
} 