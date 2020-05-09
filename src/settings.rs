const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

pub struct Settings {
    pub width: u32,
    pub height: u32,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}