extern crate bytes;
extern crate mio;
extern crate bitreader;
extern crate hpack_codec;
extern crate rustls;

mod proto;
mod app;

use app::App;

use app::message::Message;
use app::call::Call;

fn main() {
    let mut app = App::new();
    
    app.get("/", | call: &Call | -> Message {
        println!("I DID NOTHING WRONG! {:?}", call);
        Message { status: 200 }
    });

    app.start();
}
