use cgmath::{Vector2};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Action {
    Move(Direction),
    Rotate(Direction),
    None,
}

pub struct Player {
    pub pos: Vector2<f64>,
    pub speed: u32,
    pub dir: Vector2<f64>,
    pub camera: Vector2<f64>,
    pub action: Action,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector2::new(12., 9.),
            speed: 10,
            dir: Vector2::new(-1., 0.),
            camera: Vector2::new(0., 0.66),
            action: Action::None,
        }
    }
}