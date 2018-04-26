# httimple.rs [![Build Status](https://travis-ci.org/halvorboe/httimple-rs.svg?branch=master)](https://travis-ci.org/halvorboe/httimple-rs) ![Version](https://img.shields.io/crates/v/httimple.svg) ![Downloads](https://img.shields.io/crates/d/httimple.svg)
Simple HTTP 2.0 library for building APIs


### Introduction 

Httiple aims to make HTTP/2 over TLS in rust simple. It adopts a Express-like interface. 

### Implemented

- A basic implementation of the h2 standard. 
- A per stream state based handler.
- Different methods 

### Working on 

- Making the implementation of HTTP/2 work in all browsers (currently only working in chrome)
- Fix TLS errors (Don't know if this is my fault of rustls)
- Implement the missing frametypes.
- Add support for priority.
- Make the callback a future.
- Implement a propper hpack lib in rust (the one used currently seems a bit broken)
- Serving static files (started, but currently impossible to make multiple requests)

### Get started

##### 1. Install Rust.

##### 2. Create the project.
```
cargo new myserver 
```
##### 3. Add required files.
Add an index.html and a certificate in the main folder. Name the certificate folder "ca".

##### 4. Add depencies
Add this to your Cargo.toml
```
[dependencies]
httimple = "*"
```
##### 5. Write the code
Make a file containing this and name it main.rs.
```
extern crate httimple;

use httimple::app::App;

use httimple::app::message::Message;
use httimple::app::call::Call;

use httimple::helpers::file;


fn main() {
    let mut app = App::new();
    app.serve("/", | call: &Call | -> Message {
        Message::from(file("index.html"))
    });
    app.start();
}
```
##### 6. Run the code
```
cargo run --release
```


