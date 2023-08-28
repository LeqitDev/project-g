use debugger::Debugger;
use loader::load_rom;
use sdl2::{
    event::{self, Event},
    keyboard::Keycode,
    pixels::Color,
    rect::Point,
};
use std::time::Duration;

use instruction_parser::run;

pub mod cpu;
mod debugger;
pub mod display;
pub mod instruction_parser;
pub mod loader;
pub mod wrapper;

fn main() {
    let data = load_rom("P:\\Programmieren\\Rust\\project-g\\main.gb");
    println!("ROM size: {:X}", data.len());
    let mut debugger = Debugger::new(data);
    debugger.wait();

    debugger.run();
}

fn start_gui() -> Result<(), String> {
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

/* type Hex {

} */
