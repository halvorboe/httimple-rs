


enum State {
    Idle(),
    Open(),
}

struct Stream {
    state: State,
    priority: u8
}