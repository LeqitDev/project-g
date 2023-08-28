use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstructionFile {
    pub unprefixed: HashMap<String, Instruction>,
    pub cbprefixed: HashMap<String, Instruction>
}

#[derive(Debug, Deserialize)]
pub struct Instruction {
    pub mnemonic: String,
    pub bytes: usize,
    pub cycles: Vec<u8>,
    pub operands: Vec<Operands>,
    pub immediate: bool,
    pub flags: Flags,
}

#[derive(Debug, Deserialize)]
pub struct Operands {
    pub name: String,
    pub immediate: bool,
}

#[derive(Debug, Deserialize)]
pub struct Flags {
    pub Z: String,
    pub N: String,
    pub H: String,
    pub C: String,
}