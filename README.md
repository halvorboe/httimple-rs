# httimple.rs [![Build Status](https://travis-ci.org/halvorboe/httimple-rs.svg?branch=master)](https://travis-ci.org/halvorboe/httimple-rs) [![Version](https://img.shields.io/crates/v/httimple.svg)] (https://crates.io/crates/httimple) [![Downloads](https://img.shields.io/crates/d/httimple.svg)] (https://crates.io/crates/httimple) 
Simple HTTP 2.0 library 




### Introduction 

Httiple aims to make HTTP/2 over TLS in rust simple. It adopts a Express-like interface. 

### Implemented

- A basic implementation of the h2 standard. 
- A per stream state based handler.
- A simple interface.
- Continous Deployment (deploys to crates.io)

### Working on 

- Making the implementation of HTTP/2 work in all browsers (currently only working in chrome)
- Fix TLS errors (Don't know if this is my fault of rustls)
- Implement the missing frametypes.
- Add support for priority.
- Make the callback a future.
- Implement a propper hpack lib in rust (the one used currently seems a bit broken)
- Serving static files (started, but currently impossible to make multiple requests)
- Write tests for everything. My testing library I was working around does not have TLS support.
- Improve tls setup process.
- Adding an ORM and mongodb (a lot of this is done in https://github.com/halvorboe/rust-rest) to possible make it more like Django Framework.

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
```rust
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


### Dependecies

bitreader -> Handles the reading of blocks. Could be removed in favor of binary operations, but it makes the code easier to read.
mio = "0.6.14" -> Handles the async io.
rustls = "0.12.0" -> TLS
hpack_codec = "0.1.0" -> Handles decoding and encoding of headers. Seems broken to some degree.

### Tests 

To run the tests, clone this repo and run:
```
cargo test
```

### Documentation 
https://docs.rs/httimple/0.1.7/httimple/
