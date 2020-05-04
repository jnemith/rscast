use crate::player::{Action, Direction, Player};
use cgmath::{ vec2, };
use sdl2::{
    pixels::Color,
    render::WindowCanvas,
    rect::Rect,
};

#[derive(PartialEq, Debug)]
pub enum Space {
    Wall(Color),
    Empty,
}

#[derive(PartialEq)]
pub enum Side {
    NS, // North/south
    EW, // East/west
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<Vec<Space>>,
}

pub struct World {
    pub map: Map,
    pub player: Player,
}

impl World {
    pub fn new(map_str: &str) -> Self {
        World { 
            map: Map::new(map_str),
            player: Player::new(),
        }
    }

    pub fn cast(&mut self, x: usize, width: u32, height: u32, canvas: &mut WindowCanvas) {
        
        let camera_x = 2. * (x as f64 / width as f64) - 1.;
        let ray_dir = self.player.dir + self.player.camera * camera_x;

        let delta_dist = vec2(
            (1. / ray_dir.x).abs(),
            (1. / ray_dir.y).abs(),
        );

        let mut step = vec2(1, 1);
        let mut side_dist = vec2(0., 0.);
        let mut map_pos = vec2(self.player.pos.x.floor() as i32, self.player.pos.y.floor() as i32);
        if ray_dir.x < 0. {
            step.x = -1;
            side_dist.x = (self.player.pos.x - map_pos.x as f64) * delta_dist.x;
        } else {
            side_dist.x = (map_pos.x as f64 + 1. - self.player.pos.x) * delta_dist.x;
        }

        if ray_dir.y < 0. {
            step.y = -1;
            side_dist.y = (self.player.pos.y - map_pos.y as f64) * delta_dist.y;
        } else {
            side_dist.y = (map_pos.y as f64 + 1. - self.player.pos.y) * delta_dist.y;
        }

        // Begin DDA
        let mut hit = 0;
        let mut side = Side::NS;
        while hit == 0 {
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_pos.x += step.x;
                side = Side::EW;
            } else {
                side_dist.y += delta_dist.y;
                map_pos.y += step.y;
                side = Side::NS;
            }

            if let Space::Wall(_) = self.map.walls[map_pos.x as usize][map_pos.y as usize] {
                hit = 1;
            }
        }

        // Calculate distance from ray to wall
        let wall_dist = if side == Side::EW {
            (map_pos.x as f64 - self.player.pos.x + (1. - step.x as f64) / 2.) / ray_dir.x
        } else {
            (map_pos.y as f64 - self.player.pos.y + (1. - step.y as f64) / 2.) / ray_dir.y
        };

        // Calculate line to draw
        let line_height = height as f64 / wall_dist;

        let mut draw_start = (-line_height as i32 / 2) + (height as  i32 / 2);
        if draw_start < 0 {
            draw_start = 0;
        }
        let mut draw_end = (line_height as i32 / 2) + (height as  i32 / 2);
        if draw_end > height as i32 {
            draw_end = height as i32 - 1;
        }

        // Set color
        let mut color = Color::RGB(0, 0, 0);
        if let Space::Wall(v) = self.map.walls[map_pos.x as usize][map_pos.y as usize] {
            color = v;
            if side == Side::NS {
                color = Color::RGB(color.r / 2, color.g / 2, color.b / 2);
            }
        }

        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(
            x as i32,
            draw_start,
            1,
            line_height as u32
        )).expect("Draw error");
    }

    pub fn update_player(&mut self, frame_time: f64) {
        // Update player position and direction
        let move_speed = frame_time * 5.;
        let mut rot_speed = frame_time * 3.;
        
        let new_pos = vec2(
            self.player.pos.x + self.player.dir.x * move_speed,
            self.player.pos.y + self.player.dir.y * move_speed,
        );

        let mut allowed = vec2(true, true);

        if let Space::Wall(_) = self.map.walls[new_pos.x as usize][self.player.pos.y as usize] {
            allowed.x = false;
        }
        if let Space::Wall(_) = self.map.walls[self.player.pos.x as usize][new_pos.y as usize] {
            allowed.y = false;
        }

        match &self.player.action {
            Action::Move(Direction::Up) if allowed.x == true => {
                self.player.pos.x = new_pos.x;
                self.player.pos.y = new_pos.y;
            },
            Action::Move(Direction::Down) if allowed.y == true => {
                self.player.pos.x = self.player.pos.x - self.player.dir.x * move_speed;
                self.player.pos.y = self.player.pos.y - self.player.dir.y * move_speed;
            },
            Action::Rotate(dir) => {
                if let Direction::Right = *dir {
                    rot_speed = -rot_speed;
                }
                let old_dir_x = self.player.dir.x;
                self.player.dir.x =
                    (self.player.dir.x * rot_speed.cos()) - (self.player.dir.y * rot_speed.sin());
                self.player.dir.y =
                    (old_dir_x * rot_speed.sin()) + (self.player.dir.y * rot_speed.cos());

                let old_camera_x = self.player.camera.x;
                self.player.camera.x =
                    (self.player.camera.x * rot_speed.cos()) - (self.player.camera.y * rot_speed.sin());
                self.player.camera.y =
                    (old_camera_x * rot_speed.sin()) + (self.player.camera.y * rot_speed.cos());
            },
            _ => ()
        }
    }
}

impl Map {
    pub fn new(map_str: &str) -> Self {
        let walls: Vec<Vec<Space>> = map_str.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    '1' => Space::Wall(Color::RGB(255, 0, 0)),
                    '2' => Space::Wall(Color::RGB(0, 255, 0)),
                    '3' => Space::Wall(Color::RGB(0, 0, 255)),
                    '4' => Space::Wall(Color::RGB(255, 0, 212)),
                    _ => Space::Empty,
                }
            }).collect()
        }).collect();

        let m_height = walls.len();
        let m_width = walls.first().unwrap().len();
        Map {
            width: m_width,
            height: m_height,
            walls: walls,
        }
    }
}