use std::collections::HashMap;
use proto::frame::data::Data;
use proto::frame::headers::Headers;
use proto::frame::continuation::Continuation;

///
/// A call from the client
/// 

#[derive(Debug)]
pub struct Call {
    headers: HashMap<Vec<u8>, Vec<u8>>,
    data: Vec<u8>,
    end_headers: bool,
    end_stream: bool,
}

impl Call {

    ///
    /// Used internally
    /// 

    pub fn from() -> Call {
        Call { 
            headers: HashMap::new(),
            data: Vec::new(),
            end_headers: false,
            end_stream: false
        }
    }

    ///
    /// Used internally
    /// 

    pub fn is_ready(&self) -> bool {
        self.end_headers && self.end_stream
    }


    ///
    /// Used internally
    /// 


    pub fn insert_data(&mut self, data: Data) {
        self.end_stream = data.is_end_stream();
        self.data.append(&mut data.get_payload());
    }


    ///
    /// Used internally
    /// 


    pub fn insert_headers(&mut self, headers: Headers) {
        self.end_stream = headers.is_end_stream();
        self.end_headers = headers.is_end_headers();
        match headers.get_headers() {   
            Some(headers) => {
                self.insert_all(headers);
            }, 
            None => {
                println!("[ERROR] Headers not decoded...");
            }
        }
    }


    ///
    /// Used internally
    /// 


    pub fn insert_continuation(&mut self, continuation: Continuation) {
        self.end_headers = continuation.is_end_headers();
        self.insert_all(continuation.get_headers());
    }


    ///
    /// Gets the path from the call.
    /// 

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


    ///
    /// Gets the method from the call
    /// 


    pub fn method(&self) -> String {
        String::from_utf8(self.headers.get(String::from(":method").as_bytes()).unwrap().clone()).unwrap()
    }


    ///
    /// Checks if the call is of the type GET
    /// 

    pub fn is_get(&self) -> bool {
        self.method() == String::from("GET")
    }

    ///
    /// Checks if the call is of the type POST
    /// 


    pub fn is_post(&self) -> bool {
        self.method() == String::from("POST")
    }


}