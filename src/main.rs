extern crate sdl2;

mod renderer;
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

    // let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    // canvas.set_draw_color(Color::RGB(0xc0, 0xc0, 0xc0));
    // let origin = Point::new(0, 0);
    // draw_circle(&mut canvas, origin, 10);
    // canvas.present();

    let mut renderer = Renderer::new(window)?;
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // canvas.clear();
        // canvas.present();
        renderer.draw()?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
