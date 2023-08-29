use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Read},
    rc::Rc,
    sync::{Arc, Mutex},
};

use self::opcodes::{Instruction, Mnemonic};

pub mod opcodes;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    stack: Vec<u16>,
    memory: Arc<Mutex<Vec<u8>>>,
}

impl CPU {
    pub fn new(memory: &Arc<Mutex<Vec<u8>>>) -> Self {
        Self {
            registers: Registers::new(),
            pc: 0,
            sp: 0xFFFE,
            stack: vec![0; 0xFFFE],
            memory: memory.clone(),
        }
    }

    #[allow(unused_variables)]
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        self.pc += 1;

        match instruction.mnemonic {
            Mnemonic::NYI => panic!("Not yet implemented opcode: {:X}", instruction.code),
            Mnemonic::NOP => {}
            Mnemonic::ADD_A_A => {
                let a = self.registers.a;
                self.add_8b(a);
            }
            Mnemonic::ADD_A_B => {
                let b = self.registers.b;
                self.add_8b(b);
            }
            Mnemonic::ADD_A_C => {
                let c = self.registers.c;
                self.add_8b(c);
            }
            Mnemonic::ADD_A_D => {
                let d = self.registers.d;
                self.add_8b(d);
            }
            Mnemonic::ADD_A_E => {
                let e = self.registers.e;
                self.add_8b(e);
            }
            Mnemonic::ADD_A_H => {
                let h = self.registers.h;
                self.add_8b(h);
            }
            Mnemonic::ADD_A_L => {
                let l = self.registers.l;
                self.add_8b(l);
            }
            Mnemonic::ADD_A_n8 => {
                self.add_8b(self.read_addr(self.pc));
                self.pc += 1;
            }
            Mnemonic::SUB_A_B => {
                let b = self.registers.b;
                self.sub_8b(b);
            }
            Mnemonic::SUB_A_L => {
                let l = self.registers.l;
                self.sub_8b(l);
            }
            Mnemonic::SUB_A_E => {
                let e = self.registers.e;
                self.sub_8b(e);
            }
            Mnemonic::RST_00 => {
                self.restart(0x0000);
            }
            Mnemonic::RST_08 => {
                self.restart(0x0008);
            }
            Mnemonic::RST_10 => {
                self.restart(0x0010);
            }
            Mnemonic::RST_18 => {
                self.restart(0x0018);
            }
            Mnemonic::RST_20 => {
                self.restart(0x0020);
            }
            Mnemonic::RST_28 => {
                self.restart(0x0028);
            }
            Mnemonic::RST_30 => {
                self.restart(0x0030);
            }
            Mnemonic::RST_38 => {
                self.restart(0x0038);
            }
            Mnemonic::JP_a16 => {
                let address = self.next_16b();
                self.jump(address);
            }
            Mnemonic::JP_C_a16 => {
                let address = self.next_16b();
                if self.registers.f.carry {
                    self.jump(address);
                }
            }
            Mnemonic::JP_NZ_a16 => {
                let address = self.next_16b();
                if !self.registers.f.zero {
                    self.jump(address);
                }
            }
            Mnemonic::LD_A_B => {
                self.registers.a = self.registers.b;
            }
            Mnemonic::LD_A_L => {
                self.registers.a = self.registers.l;
            }
            Mnemonic::LD_A_n8 => {
                let digit = self.next();
                self.registers.a = digit;
            }
            Mnemonic::LD_H_n8 => {
                self.registers.h = self.next();
            }
            Mnemonic::LD_a16_A => {
                let address = self.next_16b();
                self.write_addr(address, self.registers.a);
            }
            Mnemonic::LD_A_DE => {
                self.registers.a = self.read_addr(self.registers.de());
            }
            Mnemonic::LD_A_a16 => {
                let address = self.next_16b();
                self.registers.a = self.read_addr(address);
            }
            Mnemonic::LD_B_H => {
                self.registers.b = self.registers.h;
            }
            Mnemonic::LD_D_L => {
                self.registers.d = self.registers.l;
            }
            Mnemonic::LD_L_E => {
                self.registers.l = self.registers.e;
            }
            Mnemonic::LD_BC_n16 => {
                let digit = self.next_16b();
                self.registers.set_bc(digit);
            }
            Mnemonic::LD_DE_n16 => {
                let digit = self.next_16b();
                self.registers.set_de(digit);
            }
            Mnemonic::LD_HL_A => {
                self.write_addr(self.registers.hl(), self.registers.a);
                self.registers.set_hl(self.registers.hl() + 1);
            }
            Mnemonic::LD_HL_n16 => {
                let digit = self.next_16b();
                self.registers.set_hl(digit);
            }
            Mnemonic::INC_DE => {
                self.registers.set_de(self.registers.de() + 1);
            }
            Mnemonic::DEC_B => {
                self.registers.b -= 1;
            }
            Mnemonic::DEC_BC => {
                self.registers.set_bc(self.registers.bc() - 1);
            }
            Mnemonic::CP_A_n8 => {
                let digit = self.next();
                self.registers.f.zero = self.registers.a == digit;
                self.registers.f.subtract = true;
                self.registers.f.carry = self.registers.a < digit;
            }
            Mnemonic::OR_A_C => {
                let new_value = self.registers.a | self.registers.c;

                self.registers.f.zero = new_value == 0;

                self.registers.a = new_value;
            }
            _ => {
                return Err(format!(
                    "; Not yet implemented routine for opcode: {:?}",
                    instruction
                ))
            }
        }
        Ok(())
    }

    fn next(&mut self) -> u8 {
        let next = self.read_addr(self.pc);
        self.pc += 1;
        next
    }

    fn next_16b(&mut self) -> u16 {
        let last = self.next();
        let first = self.next();
        ((first as u16) << 8) + last as u16
    }

    fn add_8b(&mut self, digit: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(digit);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;

        self.registers.a = new_value;
    }

    fn sub_8b(&mut self, digit: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(digit);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;

        self.registers.a = new_value;
    }

    fn restart(&mut self, address: u16) {
        self.sp -= 2;
        self.stack[self.sp as usize] = self.pc;
        self.pc = address;
    }

    fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    fn read_addr(&self, addr: u16) -> u8 {
        let mem_lock = self.memory.lock().unwrap();
        mem_lock[addr as usize]
    }

    fn write_addr(&mut self, addr: u16, value: u8) {
        if addr > 0x8000 && addr < 0x9FFF {
            // println!("Writing into vram: {:X}, {:X}", addr, value);
        }
        let mut mem_lock = self.memory.lock().unwrap();
        mem_lock[addr as usize] = value;
    }
}

pub struct Registers {
    // single registers (8bit)
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    // First 16bit register
    pub fn af(&self) -> u16 {
        ((u8::from(self.f) as u16) << 8) | self.a as u16
    }

    pub fn set_af(&mut self, v: u16) {
        self.f = Flags::from((v >> 8) as u8);
        self.a = v as u8;
    }

    // Second 16bit register
    pub fn bc(&self) -> u16 {
        ((self.c as u16) << 8) | self.b as u16
    }

    pub fn set_bc(&mut self, v: u16) {
        self.c = (v >> 8) as u8;
        self.b = v as u8;
    }

    // Third 16bit register
    pub fn de(&self) -> u16 {
        ((self.e as u16) << 8) | self.d as u16
    }

    pub fn set_de(&mut self, v: u16) {
        self.e = (v >> 8) as u8;
        self.d = v as u8;
    }

    // Fourth 16bit register
    pub fn hl(&self) -> u16 {
        ((self.l as u16) << 8) | self.h as u16
    }

    pub fn set_hl(&mut self, v: u16) {
        self.l = (v >> 8) as u8;
        self.h = v as u8;
    }

    fn new() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0.into(),
            h: 0x01,
            l: 0x4D,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

// Convert struct to u8
impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        ((flags.zero as u8) << 7)
            | ((flags.subtract as u8) << 6)
            | ((flags.half_carry as u8) << 5)
            | ((flags.carry as u8) << 4)
    }
}

// Convert u8 to struct
impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Self {
            zero: (byte >> 7) & 0b1 != 0,
            subtract: (byte >> 6) & 0b1 != 0,
            half_carry: (byte >> 5) & 0b1 != 0,
            carry: (byte >> 4) & 0b1 != 0,
        }
    }
}

impl Flags {
    pub fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }

    //Builder
    pub fn set_zero(mut self) -> Self {
        self.zero = true;
        self
    }

    pub fn set_subtract(mut self) -> Self {
        self.subtract = true;
        self
    }

    pub fn set_half_carry(mut self) -> Self {
        self.half_carry = true;
        self
    }

    pub fn set_carry(mut self) -> Self {
        self.carry = true;
        self
    }
}

struct MemoryBus {
    memory: HashMap<u16, u8>,
}
