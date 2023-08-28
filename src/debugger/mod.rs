use std::{
    cell::RefCell,
    io::{self, Read},
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{
    cpu::{opcodes::Instruction, CPU},
    display::Display,
};

pub struct Debugger {
    cpu: CPU,
    gpu: Display,
    debugging: bool,
    breakpoints: Vec<u16>,
    memory: Vec<u8>,
    skip_nops: bool,
    pause: bool,
    skipped_nops: u16,
}

impl Debugger {
    pub fn new(rom: Vec<u8>) -> Self {
        // cpu.pc = 0x100;

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

        let mem_mut = Arc::new(Mutex::new(memory.clone()));
        let cpu = CPU::new(&mem_mut);
        let gpu = Display::new(&mem_mut);

        Self {
            cpu,
            gpu,
            debugging: false,
            breakpoints: vec![/* 0x190 */],
            skip_nops: true,
            pause: false,
            skipped_nops: 0,
            memory,
        }
    }

    pub fn wait(&self) {
        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory[self.cpu.pc as usize];
            let instruction: Instruction = opcode.into();
            if !(instruction == Instruction::NOP(4)) && self.skip_nops {
                if self.skipped_nops > 0 {
                    print!("Skipped {} NOPs, ", self.skipped_nops);
                    self.skipped_nops = 0;
                }
                self.pause = true;
                let bytes = self.get_next_bytes(2);
                print!(
                    "{:X}({:?}): {}; PC: {:X}, SP: {:X}; {:?}; A: {:X}",
                    opcode,
                    instruction,
                    bytes,
                    self.cpu.pc,
                    self.cpu.sp,
                    self.cpu.registers.f,
                    self.cpu.registers.a
                );
                self.pause = false;
            } else if (instruction == Instruction::NOP(4)) && self.skip_nops {
                self.skipped_nops += 1;
            }
            if !self.breakpoints.is_empty() {
                self.breakpoints.iter().for_each(|b| {
                    if self.cpu.pc == *b {
                        self.wait();
                    }
                });
            }
            if !self.pause {
                if let Err(s) = self.cpu.execute(opcode.into()) {
                    print!("{}", s);
                }
                if !self.gpu.gpu_loop() {
                    break;
                }
            }
            if !(instruction == Instruction::NOP(4)) && self.skip_nops {
                println!("; PC: {:X}, SP: {:X}", self.cpu.pc, self.cpu.sp);
            }
        }
    }

    fn get_next_bytes(&mut self, nbytes: u8) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        let mut pc = self.cpu.pc;
        for _ in 0..nbytes {
            pc += 1;
            let opcode = self.memory[pc as usize];
            bytes.push(opcode);
        }
        bytes
            .iter()
            .map(|b| format!("{:X}", b))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
