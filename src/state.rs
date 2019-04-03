pub static COLORS: [(&str, &str); 2] = [("Black", "#000000"), ("Green", "#3DC06C")];

static DEFAULT_COLOR: &str = COLORS[0].1;

pub struct State {
    is_drawing: bool,
    color: String,
}

impl State {
    pub fn new() -> State {
        State {
            is_drawing: false,
            color: DEFAULT_COLOR.to_string(),
        }
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

    pub fn update_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }
}
