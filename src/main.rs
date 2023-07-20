use sdl2::{pixels::Color, event::{self, Event}, keyboard::Keycode, rect::Point};
use std::time::Duration;

mod cpu;
mod opcodes;
mod graphics;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_point(Point::new(100, 100))?;
    canvas.draw_point(Point::new(101, 101))?;
    canvas.draw_point(Point::new(101, 100))?;
    canvas.draw_point(Point::new(100, 101))?;
    canvas.present();
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
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
