use cgmath::{Vector2};

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum Action {
    Move(Direction),
    Rotate(Direction),
}

pub struct Player {
    pub pos: Vector2<f64>,
    pub speed: u32,
    pub dir: Vector2<f64>,
    pub camera: Vector2<f64>,
    pub actions: Vec<Action>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector2::new(12., 9.),
            speed: 7,
            dir: Vector2::new(-1., 0.),
            camera: Vector2::new(0., 0.66),
            actions: Vec::new(),
        }
    }
}