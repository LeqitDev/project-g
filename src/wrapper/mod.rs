use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use crate::{cpu::CPU, debugger::Debugger, display::Display};

pub struct GameBoy {
    // display: Display,
    // debugger: Debugger,
}

impl GameBoy {
    pub fn start(rom: Vec<u8>) -> Self {
        // Startup routine

        let mut memory = vec![0; 0xFFFF];
        memory[0xFF10] = 0x80;
        memory[0xFF11] = 0xBF;
        memory[0xFF12] = 0xF3;
        memory[0xFF14] = 0xBF;
        memory[0xFF16] = 0xF3;
        memory[0xFF19] = 0xBF;
        memory[0xFF1A] = 0x7F;
        memory[0xFF1B] = 0xFF;
        memory[0xFF1C] = 0x9F;
        memory[0xFF1E] = 0xBF;
        memory[0xFF20] = 0xFF;
        memory[0xFF23] = 0xBF;
        memory[0xFF24] = 0x77;
        memory[0xFF25] = 0xF3;
        memory[0xFF26] = 0xF1;
        memory[0xFF40] = 0x91;
        memory[0xFF47] = 0xFC;
        memory[0xFF48] = 0xFF;
        memory[0xFF49] = 0xFF;

        memory[0x00..rom.len()].copy_from_slice(&rom);

        // Share memory to cpu and display

        let mem_bus = Arc::new(Mutex::new(memory.clone()));
        let state = Arc::new(Mutex::new(State::default()));

        let mut debugger = Debugger::new(&mem_bus, &state);
        let display = Display::new(&mem_bus, &state);

        tokio::spawn(async move {
            // debugger.run().await;
        });

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 650.0]),
            ..Default::default()
        };
        let _ = eframe::run_native(
            "Multiple viewports",
            options,
            Box::new(|_cc| Box::new(display)),
        );

        Self {}
    }

    /* pub async fn run(mut self) {
        let mut dbg = self.debugger;
        let handle = tokio::spawn(async move {
            dbg.run().await;
        });

        let mut debugger = false;

        loop {
            if !self.display.gpu_loop() {
                break;
            }
            sleep(Duration::from_millis(1)).await;
        }
    } */
}

pub struct State {
    pub exit: bool,
    pub breakpoint: bool,
    pub update: bool,
    pub cpu: Option<CPU>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            exit: Default::default(),
            breakpoint: Default::default(),
            update: true,
            cpu: Default::default(),
        }
    }
}
