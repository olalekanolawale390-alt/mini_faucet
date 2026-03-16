#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Input,
    Connecting,
    Connected,
    Claiming,
    Success(String),
    Error(String),
}

pub struct App {
    pub input: String,
    pub state: AppState,
    pub cursor_position: usize,
    pub progress: f64,
    pub cursor_visible: bool,
    pub selected_button: usize,
    pub button_areas: Vec<(u16, u16, u16, u16)>,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            state: AppState::Input,
            cursor_position: 0,
            progress: 0.0,
            cursor_visible: true,
            selected_button: 0,
            button_areas: vec![],
        }
    }

    pub fn handle_char(&mut self, c: char) {
        if self.state == AppState::Input {
            self.input.insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.state == AppState::Input && self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn handle_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn handle_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn reset(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
        self.state = AppState::Input;
        self.progress = 0.0;
        self.selected_button = 0;
    }

    pub fn toggle_cursor(&mut self) {
        self.cursor_visible = !self.cursor_visible;
    }

    pub fn select_next_button(&mut self) {
        self.selected_button = (self.selected_button + 1) % 2;
    }

    pub fn select_prev_button(&mut self) {
        self.selected_button = if self.selected_button == 0 { 1 } else { 0 };
    }

    pub fn check_button_click(&mut self, x: u16, y: u16) -> Option<usize> {
        for (i, (bx, by, bw, bh)) in self.button_areas.iter().enumerate() {
            if x >= *bx && x < bx + bw && y >= *by && y < by + bh {
                return Some(i);
            }
        }
        None
    }
}
