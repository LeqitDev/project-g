use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    cpu::{opcodes::Instruction, CPU},
    debugger::Debugger,
    display::Display,
};
use async_std::task;

pub struct GameBoy {
    cpu: CPU,
    display: Display,
    memory: Vec<u8>,
    debugger: Option<Debugger>,
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
        let cpu = CPU::new(&mem_bus);
        let display = Display::new(&mem_bus);

        Self {
            cpu,
            display,
            debugger: None,
            memory,
        }
    }

    fn run(&mut self) {
        let mem = self.memory.clone();
        let mut cpu = &self.cpu;
        thread::spawn(move || loop {
            let opcode = mem[cpu.pc as usize];
            let instruction: Instruction = opcode.into();
            if let Err(s) = cpu.execute(opcode.into()) {
                print!("{}", s);
            }
        });
    }
}
