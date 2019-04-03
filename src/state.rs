pub static COLORS: [(&str, &str); 4] = [
    ("Black", "#000000"),
    ("Green", "#3DC06C"),
    ("Red", "#FF0000"),
    ("White", "#FFFFFF"),
];

static DEFAULT_COLOR: &str = COLORS[0].1;

pub static PEN_SIZES: [f64; 3] = [1.0, 2.0, 4.0];

static DEFAULT_PEN_SIZE: f64 = PEN_SIZES[0];

pub struct State {
    is_drawing: bool,
    color: String,
    pen_size: f64,
}

impl State {
    pub fn new() -> State {
        State {
            is_drawing: false,
            color: DEFAULT_COLOR.to_string(),
            pen_size: DEFAULT_PEN_SIZE,
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

    pub fn update_pen_size(&mut self, size: f64) {
        self.pen_size = size;
    }

    pub fn get_pen_size(&self) -> f64 {
        self.pen_size
    }
}
