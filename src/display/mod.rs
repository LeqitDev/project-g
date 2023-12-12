use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, SystemTime},
};

use eframe::{
    egui::{Button, Frame, Sense},
    epaint::{Color32, Pos2, Rect, Rounding, Vec2},
};
use egui::{vec2, Painter, Shape, Window};
use egui_extras::{Column, TableBuilder};

use crate::{
    cpu::{opcodes::Instruction, CPU},
    wrapper::State,
};

pub struct Display {
    state: Arc<Mutex<State>>,
    memory: Arc<Mutex<Vec<u8>>>,
    debug: bool,
    size: usize,
    prev_tilemap: Vec<u8>,
    tilemap_shapes: Vec<Shape>,
    tick: Arc<Mutex<i32>>,
    show_deferred_viewport: Arc<AtomicBool>,
}

impl Display {
    pub fn new(memory: &Arc<Mutex<Vec<u8>>>, state: &Arc<Mutex<State>>) -> Self {
        let t = Arc::new(Mutex::new(0));
        let tc = Arc::clone(&t);
        let o = Self {
            state: state.clone(),
            memory: memory.clone(),
            debug: false,
            size: 4,
            prev_tilemap: vec![],
            tilemap_shapes: vec![],
            tick: t,
            show_deferred_viewport: Arc::new(AtomicBool::default()),
        };
        let mem = memory.clone();
        let st = state.clone();
        thread::spawn(move || {
            let mut cpu = CPU::new(&mem);
            loop {
                let opcode = mem.lock().unwrap()[cpu.pc as usize];
                // let instruction: Instruction = opcode.into();
                if !st.lock().unwrap().breakpoint {
                    if let Err(s) = cpu.execute(opcode.into()) {
                        print!("{}", s);
                    }
                }
                if st.lock().unwrap().exit {
                    break;
                }
                /* *tc.lock().unwrap() += 1;
                std::thread::sleep(Duration::from_millis(10)); */
            }
        });
        o
    }

    /* pub fn gpu_loop(&mut self) -> bool {
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

        /* for event in self.event_pump.poll_iter() {
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
        } */
        // println!("gpu update: tiles: {}, lcdc: {:?}", tiles.len(), lcdc);

        tilemap.iter().enumerate().for_each(|(i, t)| {
            /* if *t == 0 {
                return;
            } */
            tiles[*t as usize]
                .draw_offset(&ui, (i as i32 % 32 * 8, i as i32 / 32 * 8));
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
    } */

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
            tiles.push(Tile::load_from_mem(tile, self.size as u8));
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

impl eframe::App for Display {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = SystemTime::now();
        let mut mem_lock = self.memory.lock().unwrap();
        let lcdc: LCDC = mem_lock[0xFF40].into();

        if !lcdc.lcd_enabled {
            return;
        }

        mem_lock[0xFF44] += 1;
        if mem_lock[0xFF44] > 153 {
            mem_lock[0xFF44] = 0;
        }

        let tilemap = mem_lock[0x9800..0x9BFF].to_vec();

        egui::TopBottomPanel::top("MyPanel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(Button::new("-")).clicked() {
                    self.state.lock().unwrap().update = true;
                    self.size -= 1;
                }

                ui.label(format!("{}", self.size));

                if ui.add(Button::new("+")).clicked() {
                    self.state.lock().unwrap().update = true;
                    self.size += 1;
                }

                ui.add_space(8.0);

                if ui.add(Button::new("Stop")).clicked() {
                    self.state.lock().unwrap().exit = true;
                }

                if ui.add(Button::new("Deferred window")).clicked() {
                    self.show_deferred_viewport.store(true, Ordering::Relaxed);
                }
            });
        });

        // if self.state.lock().unwrap().update {
        let tiles = self.load_objects(mem_lock[0x9000..0x9800].to_vec());
        // }

        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (mut response, painter) = ui.allocate_painter(
                    Vec2::new((160 * self.size) as f32, (144 * self.size) as f32),
                    Sense::hover(),
                );

                painter.rect_filled(response.rect, Rounding::ZERO, Color32::WHITE);

                /* painter.rect_filled(
                    get_pixel_dirty_position(&response.rect.left_top(), Vec2::splat(1.), self.size),
                    Rounding::ZERO,
                    Color32::from_rgb(255, 0, 0),
                    // Stroke::new(1.0, Color32::from_rgb(255, 0, 0)),
                ); */
                let mut tilemap_shapes: Vec<Shape> = vec![];
                if tilemap != self.prev_tilemap || self.state.lock().unwrap().update {
                    tilemap.iter().enumerate().for_each(|(i, t)| {
                        /* if *t == 0 {
                            return;
                        } */
                        tilemap_shapes.append(&mut tiles[*t as usize].draw_offset(
                            &response.rect.left_top(),
                            (i as i32 % 32 * 8, i as i32 / 32 * 8),
                        ));
                    });
                    self.prev_tilemap = tilemap;
                    self.tilemap_shapes = tilemap_shapes;
                    if self.state.lock().unwrap().update {
                        self.state.lock().unwrap().update = false;
                    }
                }
                painter.extend(self.tilemap_shapes.clone());

                response
            });

            match now.elapsed() {
                Ok(elapsed) => {
                    // it prints '2'
                    println!("{}", elapsed.as_millis());
                }
                Err(e) => {
                    // an error occurred!
                    println!("Error: {e:?}");
                }
            }

            ctx.request_repaint();

            /* ui.checkbox(
                &mut self.show_immediate_viewport,
                "Show immediate child viewport",
            );

            let mut show_deferred_viewport = self.show_deferred_viewport.load(Ordering::Relaxed);
            ui.checkbox(&mut show_deferred_viewport, "Show deferred child viewport");
            self.show_deferred_viewport
                .store(show_deferred_viewport, Ordering::Relaxed); */
            // self.tick += 1;
        });

        let mem_cpy = self.memory.clone();

        if ctx.input(|i| i.viewport().close_requested()) {
            self.state.lock().unwrap().exit = true;
        }

        if self.show_deferred_viewport.load(Ordering::Relaxed) {
            let show_deferred_viewport = self.show_deferred_viewport.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("debug_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Debugger")
                    .with_inner_size([400.0, 600.0]),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from deferred viewport");

                        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;

                        let mut table = TableBuilder::new(ui)
                            .striped(true)
                            .resizable(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .column(Column::initial(100.0).range(40.0..=300.0))
                            .column(Column::initial(100.0).at_least(40.0).clip(true))
                            .column(Column::remainder())
                            .min_scrolled_height(0.0);

                        table
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("Row");
                                });
                                header.col(|ui| {
                                    ui.strong("Expanding content");
                                });
                                header.col(|ui| {
                                    ui.strong("Clipped text");
                                });
                                header.col(|ui| {
                                    ui.strong("Content");
                                });
                            })
                            .body(|mut body| {
                                body.rows(
                                    text_height,
                                    mem_cpy.lock().unwrap().len(),
                                    |row_index, mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("{:X}", row_index));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!(
                                                "{:X}",
                                                mem_cpy.lock().unwrap()[row_index]
                                            ));
                                        });
                                        row.col(|ui| {
                                            ui.label("klfnelknf");
                                        });
                                        row.col(|ui| {
                                            ui.add(
                                                egui::Label::new(
                                                    "Thousands of rows of even height",
                                                )
                                                .wrap(false),
                                            );
                                        });
                                    },
                                );
                            });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent to close us.
                        show_deferred_viewport.store(false, Ordering::Relaxed);
                    }
                },
            );
        }
    }
}

pub struct Tile {
    pixels: Vec<Color32>,
    size: u8,
}

impl Tile {
    fn get_pixel_dirty_position(&self, left_top: &Pos2, coords: Vec2) -> Rect {
        self.get_pixel_position(left_top, coords * Vec2::splat(self.size as f32))
    }

    fn get_pixel_position(&self, left_top: &Pos2, coords: Vec2) -> Rect {
        Rect::from_two_pos(
            *left_top + coords,
            *left_top + (coords + Vec2::splat(self.size as f32)),
        )
    }

    fn load_from_mem(mem: Vec<u16>, size: u8) -> Self {
        let mut s = Self {
            pixels: vec![Color32::RED; 8 * 8],
            size,
        };

        mem.iter().enumerate().for_each(|(y, l)| {
            let pixels = s.split_u16_into_2bit_numbers(*l);
            pixels.iter().enumerate().for_each(|(x, c)| {
                if *c != 0 {
                    s.pixels[y * 8 + x] = match c {
                        1 => Color32::from_rgb(221, 228, 231),
                        2 => Color32::from_rgb(55, 71, 79),
                        3 => Color32::BLACK,
                        _ => Color32::RED,
                    }
                }
            })
        });

        s
    }

    fn draw_offset(&self, rect: &Pos2, offset: (i32, i32)) -> Vec<Shape> {
        let mut ret: Vec<Shape> = vec![];
        if self.pixels.is_empty() {
            return ret;
        }
        for y in 0..8 {
            for x in 0..8 {
                let pixel = self.pixels[y * 8 + x];
                if pixel == Color32::RED {
                    continue;
                }
                ret.push(Shape::rect_filled(
                    self.get_pixel_dirty_position(
                        rect,
                        vec2(offset.0 as f32 + x as f32, offset.1 as f32 + y as f32),
                    ),
                    Rounding::ZERO,
                    pixel,
                ));
            }
        }
        ret
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

/* struct DebugDisplay {
    // canvas: Canvas<Window>,
}

impl DebugDisplay {
    fn new(sdl_context: &Sdl) -> Self {
        /* let video_subsystem = sdl_context.video().unwrap();

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
        */
        Self {}
    }

    fn debug_loop(&mut self) -> bool {
        // self.canvas.present();

        true
    }
} */
