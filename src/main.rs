extern crate sdl2;

mod renderer;
use crate::renderer::scene::Vec3;
use renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Ray tracer", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut position = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -20.0,
    };

    const MOVE_UNIT: f32 = 1.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(code) => match code {
                        Keycode::A => {
                            position.x -= MOVE_UNIT;
                        }
                        Keycode::D => {
                            position.x += MOVE_UNIT;
                        }
                        Keycode::W => {
                            position.z += MOVE_UNIT;
                        }
                        Keycode::S => {
                            position.z -= MOVE_UNIT;
                        }
                        _ => {}
                    },
                    None => {}
                },
                _ => {}
            }
        }

        renderer.draw(&position)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
