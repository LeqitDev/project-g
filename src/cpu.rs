pub struct CPU {
    registers: Registers,
    flags: Flags,
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn af(&self) -> u16 {
        ((self.f as u16) << 8) | self.a as u16
    }

    fn set_af(&mut self, v: u16) {
        self.f = (v >> 8) as u8;
        self.a = v as u8;
    }
    
    fn bc(&self) -> u16 {
        ((self.c as u16) << 8) | self.b as u16
    }

    fn set_bc(&mut self, v: u16) {
        self.c = (v >> 8) as u8;
        self.b = v as u8;
    }
    
    fn de(&self) -> u16 {
        ((self.e as u16) << 8) | self.d as u16
    }

    fn set_de(&mut self, v: u16) {
        self.e = (v >> 8) as u8;
        self.d = v as u8;
    }
    
    fn hl(&self) -> u16 {
        ((self.l as u16) << 8) | self.h as u16
    }

    fn set_hl(&mut self, v: u16) {
        self.l = (v >> 8) as u8;
        self.h = v as u8;
    }
}

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        ((flags.zero as u8) << 7) |
        ((flags.subtract as u8) << 6) |
        ((flags.half_carry as u8) << 5) |
        ((flags.carry as u8) << 4)
    }
}

impl From<u8> for Flags {
    fn from(dbyte: u8) -> Self {
        Flags {
            zero: (dbyte >> 7) & 0b1 != 0,
            subtract: (dbyte >> 6) & 0b1 != 0,
            half_carry: (dbyte >> 5) & 0b1 != 0,
            carry: (dbyte >> 4) & 0b1 != 0
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