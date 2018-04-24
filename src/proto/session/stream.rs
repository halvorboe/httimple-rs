


enum State {
    Idle(),
    Open(),
}

pub struct Stream {
    state: State,
    stream_dependency: u32,
    priority: u8
}