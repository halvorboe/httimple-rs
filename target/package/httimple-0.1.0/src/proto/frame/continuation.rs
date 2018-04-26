use proto::frame::head::Head;
use proto::frame::headers::parse_header_block_fragment;

use std::collections::HashMap;


#[derive(Debug)]
pub struct Continuation {
    head: Head,
    inner: HashMap<Vec<u8>, Vec<u8>>
}


impl Continuation {

    pub fn from(head: Head, buf: Vec<u8>) -> Continuation {
        let inner = parse_header_block_fragment(&buf);
        Continuation { head: head, inner: inner}
    }

    pub fn is_end_headers(&self) -> bool {
        self.head.has_flag(2)
    } 

    pub fn get_headers(&self) -> HashMap<Vec<u8>, Vec<u8>> {
        self.inner.clone()
    } 

}