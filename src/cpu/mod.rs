use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Read},
    rc::Rc,
    sync::{Arc, Mutex},
};

use self::opcodes::Instruction;

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

        match instruction {
            Instruction::NYI(byte) => panic!("Not yet implemented opcode: {:X}", byte),
            Instruction::NOP(ticks) => {}
            Instruction::ADD_A_A(ticks) => {
                let a = self.registers.a;
                self.add_8b(a);
            }
            Instruction::ADD_A_B(ticks) => {
                let b = self.registers.b;
                self.add_8b(b);
            }
            Instruction::ADD_A_C(ticks) => {
                let c = self.registers.c;
                self.add_8b(c);
            }
            Instruction::ADD_A_D(ticks) => {
                let d = self.registers.d;
                self.add_8b(d);
            }
            Instruction::ADD_A_E(ticks) => {
                let e = self.registers.e;
                self.add_8b(e);
            }
            Instruction::ADD_A_H(ticks) => {
                let h = self.registers.h;
                self.add_8b(h);
            }
            Instruction::ADD_A_L(ticks) => {
                let l = self.registers.l;
                self.add_8b(l);
            }
            Instruction::ADD_A_n8(ticks, bytes) => {
                self.add_8b(self.read_addr(self.pc));
                self.pc += 1;
            }
            Instruction::SUB_A_B(ticks) => {
                let b = self.registers.b;
                self.sub_8b(b);
            }
            Instruction::SUB_A_L(ticks) => {
                let l = self.registers.l;
                self.sub_8b(l);
            }
            Instruction::SUB_A_E(ticks) => {
                let e = self.registers.e;
                self.sub_8b(e);
            }
            Instruction::RST_00(ticks) => {
                self.restart(0x0000);
            }
            Instruction::RST_08(ticks) => {
                self.restart(0x0008);
            }
            Instruction::RST_10(ticks) => {
                self.restart(0x0010);
            }
            Instruction::RST_18(ticks) => {
                self.restart(0x0018);
            }
            Instruction::RST_20(ticks) => {
                self.restart(0x0020);
            }
            Instruction::RST_28(ticks) => {
                self.restart(0x0028);
            }
            Instruction::RST_30(ticks) => {
                self.restart(0x0030);
            }
            Instruction::RST_38(ticks) => {
                self.restart(0x0038);
            }
            Instruction::JP_a16(ticks, bytes) => {
                let address = self.next_16b();
                self.jump(address);
            }
            Instruction::JP_C_a16(ticks, bytes) => {
                let address = self.next_16b();
                if self.registers.f.carry {
                    self.jump(address);
                }
            }
            Instruction::JP_NZ_a16(ticks, bytes) => {
                let address = self.next_16b();
                if !self.registers.f.zero {
                    self.jump(address);
                }
            }
            Instruction::LD_A_B(ticks) => {
                self.registers.a = self.registers.b;
            }
            Instruction::LD_A_L(ticks) => {
                self.registers.a = self.registers.l;
            }
            Instruction::LD_A_n8(ticks, bytes) => {
                let digit = self.next();
                self.registers.a = digit;
            }
            Instruction::LD_H_n8(ticks, bytes) => {
                self.registers.h = self.next();
            }
            Instruction::LD_a16_A(ticks, bytes) => {
                let address = self.next_16b();
                self.write_addr(address, self.registers.a);
            }
            Instruction::LD_A_DE(ticks) => {
                self.registers.a = self.read_addr(self.registers.de());
            }
            Instruction::LD_A_a16(ticks, bytes) => {
                let address = self.next_16b();
                self.registers.a = self.read_addr(address);
            }
            Instruction::LD_B_H(ticks) => {
                self.registers.b = self.registers.h;
            }
            Instruction::LD_D_L(ticks) => {
                self.registers.d = self.registers.l;
            }
            Instruction::LD_L_E(ticks) => {
                self.registers.l = self.registers.e;
            }
            Instruction::LD_BC_n16(ticks, bytes) => {
                let digit = self.next_16b();
                self.registers.set_bc(digit);
            }
            Instruction::LD_DE_n16(ticks, bytes) => {
                let digit = self.next_16b();
                self.registers.set_de(digit);
            }
            Instruction::LD_HL_A(ticks) => {
                self.write_addr(self.registers.hl(), self.registers.a);
                self.registers.set_hl(self.registers.hl() + 1);
            }
            Instruction::LD_HL_n16(ticks, bytes) => {
                let digit = self.next_16b();
                self.registers.set_hl(digit);
            }
            Instruction::INC_DE(ticks) => {
                self.registers.set_de(self.registers.de() + 1);
            }
            Instruction::DEC_B(ticks) => {
                self.registers.b -= 1;
            }
            Instruction::DEC_BC(ticks) => {
                self.registers.set_bc(self.registers.bc() - 1);
            }
            Instruction::CP_A_n8(ticks, bytes) => {
                let digit = self.next();
                self.registers.f.zero = self.registers.a == digit;
                self.registers.f.subtract = true;
                self.registers.f.carry = self.registers.a < digit;
            }
            Instruction::OR_A_C(ticks) => {
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
