pub struct State {
    is_drawing: bool,
}

impl State {
    pub fn new() -> State {
        State { is_drawing: false }
    }

    pub fn start_drawing(&mut self) {
        self.is_drawing = true;
    }

    pub fn stop_drawing(&mut self) {
        self.is_drawing = false;
    }

    pub fn is_drawing(&self) -> bool {
        self.is_drawing
    }
}
