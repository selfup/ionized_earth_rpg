#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub idle: bool,

    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub animation_index: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,

            idle: true,
            left: false,
            right: false,
            up: false,
            down: false,

            animation_index: 0,
        }
    }

    pub fn set_all_to_false(&mut self) {
        self.idle = false;
        self.left = false;
        self.right = false;
        self.up = false;
        self.down = false;
    }
}
