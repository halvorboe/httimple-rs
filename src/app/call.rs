use std::collections::HashMap;
use proto::frame::data::Data;
use proto::frame::headers::Headers;
use proto::frame::continuation::Continuation;


#[derive(Debug)]
pub struct Call {
    headers: HashMap<Vec<u8>, Vec<u8>>,
    data: Vec<u8>,
    end_headers: bool,
    end_stream: bool,
}

impl Call {

    pub fn from() -> Call {
        Call { 
            headers: HashMap::new(),
            data: Vec::new(),
            end_headers: false,
            end_stream: false
        }
    }

    pub fn is_ready(&self) -> bool {
        self.end_headers && self.end_stream
    }

    pub fn insert_data(&mut self, data: Data) {
        self.end_stream = data.is_end_stream();
        self.data.append(&mut data.get_payload());
    }

    pub fn insert_headers(&mut self, headers: Headers) {
        self.end_stream = headers.is_end_stream();
        self.end_headers = headers.is_end_headers();
        self.insert_all(headers.get_headers());
    }

    pub fn insert_continuation(&mut self, continuation: Continuation) {
        self.end_headers = continuation.is_end_headers();
        self.insert_all(continuation.get_headers());
    }

    pub fn path(&self) -> Option<&Vec<u8>> {
        for (key, value) in &self.headers {
            println!("{:?} {:?}", String::from_utf8(key.clone()).unwrap(), String::from_utf8(value.clone()).unwrap());
        }
        println!("{:?}", self.headers.get(String::from(":path").as_bytes()));
        self.headers.get(String::from(":path").as_bytes())
    }

    // Helper 

    fn insert_all(&mut self, headers: HashMap<Vec<u8>, Vec<u8>>) {
        for (key, value) in &headers {
            self.headers.insert(key.clone(), value.clone());
        }
    }


}