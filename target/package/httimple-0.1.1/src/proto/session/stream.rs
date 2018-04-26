

#[allow(dead_code)]
enum State {
    Idle(),
    Open(),
}

#[allow(dead_code)]
pub struct Stream {
    state: State,
    stream_dependency: u32,
    priority: u8
}