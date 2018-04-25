extern crate bytes;
extern crate mio;
extern crate bitreader;
extern crate hpack_codec;
extern crate rustls;

mod proto;
mod server;

use server::Server;

use server::message::Message;
use server::call::Call;

fn main() {
    let mut serv = Server::new();

    fn callback(call: &Call) -> Message {
        println!("I DID NOTHING WRONG! {:?}", call);
        Message { status: 200 }
    }
    
    serv.get("/", callback);

    serv.go();
}
