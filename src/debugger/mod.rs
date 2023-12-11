use std::{
    cell::RefCell,
    io::{self, Read, Write},
    rc::Rc,
    sync::{Arc, Mutex},
};

use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    cpu::{opcodes::Instruction, CPU},
    display::Display,
    wrapper::State,
};

use self::display::DebugDisplay;

mod display;

pub struct Debugger {
    cpu: CPU,
    state: Arc<Mutex<State>>,
    debugging: bool,
    breakpoints: Vec<u16>,
    memory: Arc<Mutex<Vec<u8>>>,
    stop: bool,
}

impl Debugger {
    pub fn new(memory: &Arc<Mutex<Vec<u8>>>, state: &Arc<Mutex<State>>) -> Self {
        let cpu = CPU::new(memory);

        Self {
            cpu,
            state: state.clone(),
            debugging: true,
            breakpoints: vec![0x100, 0x150, 0x152],
            stop: false,
            memory: memory.clone(),
        }
    }

    pub fn wait(&self) {
        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }

    pub async fn run(&mut self) {
        let mut stepping = false;

        loop {
            /* if self.debugging && self.stop {
                print!("> ");

                let line = read_line().unwrap();

                println!("{}", line);

                match line.as_str() {
                    "" => self.stop = false,
                    "s" => stepping = true,
                    "o" => stepping = false,
                    "n" => self.breakpoints = self.breakpoints.split_at(1).1.to_vec(),
                    "q" => break,
                    _ => {}
                }
            } */

            if self.state.lock().unwrap().exit {
                break;
            }

            let opcode = self.memory.lock().unwrap()[self.cpu.pc as usize];
            let instruction: Instruction = opcode.into();

            if !self.breakpoints.is_empty() {
                self.breakpoints.iter().for_each(|b| {
                    if self.cpu.pc == *b {
                        self.stop = true;
                    }
                });
            }

            if self.debugging && self.stop {
                let str = match instruction.length {
                    0 => format!("{:X}\t{:?}", instruction.code, instruction.mnemonic),
                    1 => {
                        let arg = self.memory.lock().unwrap()[self.cpu.pc as usize + 1];
                        format!(
                            "{:X} {:X}\t{:?} {:X}",
                            instruction.code, arg, instruction.mnemonic, arg
                        )
                    }
                    2 => {
                        let arg1 = self.memory.lock().unwrap()[self.cpu.pc as usize + 1];
                        let arg2 = self.memory.lock().unwrap()[self.cpu.pc as usize + 2];
                        let arg = (arg2 as u16) << 8 | arg1 as u16;
                        format!(
                            "{:X} {:X} {:X}\t{:?} {:X}",
                            instruction.code, arg1, arg2, instruction.mnemonic, arg
                        )
                    }
                    _ => "".to_string(),
                };
                println!(">>\t{:X}: {}", self.cpu.pc, str);

                let event = read().unwrap();

                if event == Event::Key(KeyCode::Char(' ').into()) {
                    self.stop = false;
                } else if event == Event::Key(KeyCode::Char('q').into()) {
                    println!("Shutting down...");
                    break;
                }
            }

            if !self.debugging || !self.stop {
                if let Err(s) = self.cpu.execute(opcode.into()) {
                    print!("{}", s);
                }
            }

            /* if stepping {
                self.stop = true;
            } */
        }
    }

    fn get_next_bytes(&mut self, nbytes: u8) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        let mut pc = self.cpu.pc;
        for _ in 0..nbytes {
            pc += 1;
            let opcode = self.memory.lock().unwrap()[pc as usize];
            bytes.push(opcode);
        }
        bytes
            .iter()
            .map(|b| format!("{:X}", b))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
