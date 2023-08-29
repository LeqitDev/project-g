use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump, Sdl,
};

use crate::wrapper::State;

pub struct Display {
    state: Arc<Mutex<State>>,
    memory: Arc<Mutex<Vec<u8>>>,
    canvas: Canvas<Window>,
    debug_display: DebugDisplay,
    debug: bool,
    event_pump: EventPump,
    size: u8,
}

impl Display {
    pub fn new(memory: &Arc<Mutex<Vec<u8>>>, state: &Arc<Mutex<State>>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CubeCoder's GameBoy", 160 * 4, 144 * 4)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.present();

        let dbg_display = DebugDisplay::new(&sdl_context);

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            state: state.clone(),
            memory: memory.clone(),
            canvas,
            event_pump,
            debug_display: dbg_display,
            debug: false,
            size: 4,
        }
    }

    pub fn gpu_loop(&mut self) -> bool {
        let mut mem_lock = self.memory.lock().unwrap();
        let lcdc: LCDC = mem_lock[0xFF40].into();

        if !lcdc.lcd_enabled {
            return true;
        }

        mem_lock[0xFF44] += 1;
        if mem_lock[0xFF44] > 153 {
            mem_lock[0xFF44] = 0;
        }

        let tiles = self.load_objects(mem_lock[0x9000..0x9800].to_vec());
        let tilemap = mem_lock[0x9800..0x9BFF].to_vec();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.state.lock().unwrap().exit = true;
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    self.debug = true;
                }
                _ => {}
            }
        }
        // println!("gpu update: tiles: {}, lcdc: {:?}", tiles.len(), lcdc);

        tilemap.iter().enumerate().for_each(|(i, t)| {
            /* if *t == 0 {
                return;
            } */
            tiles[*t as usize]
                .draw_offset(&mut self.canvas, (i as i32 % 32 * 8, i as i32 / 32 * 8));
        });

        /* tiles.iter().enumerate().for_each(|(i, t)| {
            t.draw_offset(
                &mut self.canvas,
                (i as i32 % (160 / 8) * 8, i as i32 / (160 / 8) * 8),
            );
        }); */

        self.canvas.present();

        if self.debug && !self.debug_display.debug_loop() {
            self.debug = false;
        }

        // ::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));
        // tiles[9].draw(&mut self.canvas);

        true
    }

    fn load_objects(&self, vram: Vec<u8>) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::new();
        for tile_id in 0..128 {
            let address = tile_id * 16;
            let mut tile: Vec<u16> = Vec::new();
            for line in 0..8 {
                let least = self.prefix_bits(vram[address + (line * 2)]);
                let first = self.suffix_bits(vram[address + (line * 2) + 1]);

                // println!("{:#018b}\n{:#018b}", first, least);

                tile.push(least | first);
            }
            tiles.push(Tile::load_from_mem(tile, self.size));
        }
        tiles
    }

    fn suffix_bits(&self, u8_value: u8) -> u16 {
        let mut u16_result = 0;

        for i in 0..8 {
            if (u8_value & (1 << i)) != 0 {
                u16_result |= 1 << (i * 2 + 1);
            }
        }

        u16_result
    }

    fn prefix_bits(&self, u8_value: u8) -> u16 {
        let mut u16_result = 0;

        for i in 0..8 {
            if (u8_value & (1 << i)) != 0 {
                u16_result |= 1 << (i * 2);
            }
        }

        u16_result
    }
}

pub struct Tile {
    pixels: Vec<Color>,
    size: u8,
}

impl Tile {
    fn load_from_mem(mem: Vec<u16>, size: u8) -> Self {
        let mut s = Self {
            pixels: vec![Color::RED; 8 * 8],
            size,
        };

        mem.iter().enumerate().for_each(|(y, l)| {
            let pixels = s.split_u16_into_2bit_numbers(*l);
            pixels.iter().enumerate().for_each(|(x, c)| {
                if *c != 0 {
                    s.pixels[y * 8 + x] = match c {
                        1 => Color::RGB(221, 228, 231),
                        2 => Color::RGB(55, 71, 79),
                        3 => Color::BLACK,
                        _ => Color::RED,
                    }
                }
            })
        });

        s
    }

    fn draw_offset(&self, canvas: &mut Canvas<Window>, offset: (i32, i32)) {
        if self.pixels.is_empty() {
            return;
        }
        for y in 0..8 {
            for x in 0..8 {
                let pixel = self.pixels[y * 8 + x];
                if pixel == Color::RED {
                    continue;
                }
                canvas.set_draw_color(pixel);
                let rect = Rect::new(
                    (offset.0 + x as i32) * self.size as i32,
                    (offset.1 + y as i32) * self.size as i32,
                    self.size as u32,
                    self.size as u32,
                );
                canvas.fill_rect(rect).expect("Failed to draw pixel");
            }
        }
    }

    fn split_u16_into_2bit_numbers(&self, u16_value: u16) -> [u8; 8] {
        let mut result = [0u8; 8];

        result.iter_mut().enumerate().for_each(|(i, b)| {
            *b = ((u16_value >> (i * 2)) & 0b11) as u8;
        });

        result.reverse();

        result
    }
}

#[derive(Debug)]
pub struct LCDC {
    lcd_enabled: bool,
    w_tm_area: bool,
    w_enabled: bool,
    bg_w_td_area: bool,
    bg_tm_area: bool,
    obj_size: bool,
    obj_enabled: bool,
    bg_w_enabled: bool,
}

impl From<u8> for LCDC {
    fn from(byte: u8) -> Self {
        Self {
            lcd_enabled: (byte >> 7) & 0b1 != 0,
            w_tm_area: (byte >> 6) & 0b1 != 0,
            w_enabled: (byte >> 5) & 0b1 != 0,
            bg_w_td_area: (byte >> 4) & 0b1 != 0,
            bg_tm_area: (byte >> 3) & 0b1 != 0,
            obj_size: (byte >> 2) & 0b1 != 0,
            obj_enabled: (byte >> 1) & 0b1 != 0,
            bg_w_enabled: byte & 0b1 != 0,
        }
    }
}

struct DebugDisplay {
    canvas: Canvas<Window>,
}

impl DebugDisplay {
    fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CubeCoder's GameBoy - Debug", 160 * 4, 144 * 4)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.present();

        Self { canvas }
    }

    fn debug_loop(&mut self) -> bool {
        self.canvas.present();

        true
    }
}
