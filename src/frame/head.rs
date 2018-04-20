

pub enum Type {
    Settings(u8) 
}

// Map number to type


pub struct Head {
    length: u32,
    type: Type,
    flags: [8: bool],
    stream_id: u32
}

impl Head {
    pub fn len(self) -> u32 {
        self.length
    }
}

/*

24 bits for lenght 
8 for type 
*/