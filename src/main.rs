mod world;
mod player;

use crate::player::{Action, Direction};

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
};
use std::time::{SystemTime, UNIX_EPOCH};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("rscast", WIDTH, HEIGHT)
        .resizable()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl.event_pump()?;

    let map_str = "111111111111111111111111
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000020000000000000001
100000020000000000000001
100000020000000000000001
122222220000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000330000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
100000000000000000000001
111111111111111111111111";

    let mut world = world::World::new(map_str);

    let mut time = get_time();
    let mut old_time;
    let mut frame_time = 0.;

    canvas.set_draw_color(Color::RGB(102, 102, 102));
    canvas.clear();
    canvas.present();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },

                // Arrow keys
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, ..} => {
                    world.player.new_action(Action::Move(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, ..} => {
                    world.player.new_action(Action::Move(Direction::Down)); 
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, ..} => {
                    world.player.new_action(Action::Rotate(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, ..} => {
                    world.player.new_action(Action::Rotate(Direction::Right));
                },

                // Arrow keys - key up
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, ..} => {
                    world.player.remove_action(Action::Move(Direction::Up));
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, ..} => {
                    world.player.remove_action(Action::Move(Direction::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, ..} => {
                    world.player.remove_action(Action::Rotate(Direction::Left));
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, ..} => {
                    world.player.remove_action(Action::Rotate(Direction::Right));
                },
                _ => (),
            }
        }

        canvas.set_draw_color(Color::RGB(198, 201, 245));
        canvas.clear();

        world.update_player(frame_time);
        
        for x in 0..WIDTH {
            world.cast(x as usize, WIDTH, HEIGHT, &mut canvas);
        }
        canvas.present();

        old_time = time;
        time = get_time();
        frame_time = (time - old_time) as f64 / 1_000_000_000.;
        // println!("{}", 1. / frame_time);
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn get_time() -> u128 {
    // Returns current time
    SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_nanos()
}