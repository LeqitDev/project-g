use std::collections::HashMap;

pub struct Opcode {
    pub code: u8,
    pub cmd: String,
    pub length: usize, // in bytes
    // TODO: add executable code or other option (match statement)
}

pub struct OpcodeIndex {
    pub entry: HashMap<u8, Opcode>,
}