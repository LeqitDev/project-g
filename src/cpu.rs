use crate::opcodes::Instruction;

pub struct CPU {
    registers: Registers,
    flags: Flags,
}

pub struct Registers { // single registers (8bit)
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    // First 16bit register
    pub fn af(&self) -> u16 {
        ((self.f as u16) << 8) | self.a as u16
    }

    pub fn set_af(&mut self, v: u16) {
        self.f = (v >> 8) as u8;
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

    pub fn execute(instruction: Instruction) {
        match instruction {
            
        }
    }
}

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

// Convert struct to u8
impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        ((flags.zero as u8) << 7) |
        ((flags.subtract as u8) << 6) |
        ((flags.half_carry as u8) << 5) |
        ((flags.carry as u8) << 4)
    }
}

// Convert u8 to struct
impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags {
            zero: (byte >> 7) & 0b1 != 0,
            subtract: (byte >> 6) & 0b1 != 0,
            half_carry: (byte >> 5) & 0b1 != 0,
            carry: (byte >> 4) & 0b1 != 0
        }
    }
}

impl Flags {
    pub fn new() -> Self {
        Flags { zero: false, subtract: false, half_carry: false, carry: false }
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
