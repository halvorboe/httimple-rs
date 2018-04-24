use proto::frame::head::Head;
use proto::util;

use std::collections::HashMap;


#[derive(Debug)]
pub struct Continuation {
    head: Head,
    inner: HashMap<String, String>
}


impl Continuation {

    pub fn from(head: Head, buf: Vec<u8>) -> Continuation {
        let inner = util::parse_header_block_fragment(&buf);
        Continuation { head: head, inner: inner}
    }

}