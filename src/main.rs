extern crate mio;
extern crate bitreader;
extern crate hpack_codec;
extern crate rustls;

mod proto;
mod app;
mod helpers;

use app::App;

use app::message::Message;
use app::call::Call;

use helpers::file;


fn main() {
    let mut app = App::new();
    app.serve("/", | call: &Call | -> Message {
        if call.is_get() {
            Message::from(file("i.html"))
        } else {
            Message::from(file("index.html"))
        }
    });
    app.start();
}
