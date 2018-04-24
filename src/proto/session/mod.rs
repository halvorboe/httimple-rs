mod stream;

use self::stream::Stream;

use proto::frame::settings::Setting;
use std::collections::HashMap;


pub struct Session {
    pub accepted: bool, // Handeshake is done
    pub settings: HashMap<u16, u32>,
    pub streams: HashMap<u32, Stream>
}

impl Session {

    pub fn is_accepted(&self) -> bool {
        return self.accepted;
    }

    pub fn accept(&mut self) {
        self.accepted = true;
    }

}