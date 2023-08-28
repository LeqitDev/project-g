use std::{fs::File, io::Read};

use self::structs::InstructionFile;


mod structs;

pub fn run() {
    let mut file = File::open("P:\\Programmieren\\Rust\\project-g\\Opcodes.json").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read contents");

    let f: InstructionFile = serde_json::from_str(&contents).expect("Failed to parse contents");

    println!("{:#?}", f.unprefixed.get("0x00").unwrap());

    for (h, i) in f.unprefixed.iter() {
        let suffix = i.operands.iter().map(|o| o.name.clone()).collect::<Vec<String>>().join("_");
        println!("{}(u8{}),", if !suffix.is_empty() { format!("{}_{}", i.mnemonic, suffix) } else { i.mnemonic.clone() }, if i.bytes > 1 {", u8"} else {""});
    }

    println!("------------");

    for (h, i) in f.unprefixed.iter() {
        let suffix = i.operands.iter().map(|o| o.name.clone()).collect::<Vec<String>>().join("_");
        println!("{} => Self::{}({}{}),", h, if !suffix.is_empty() { format!("{}_{}", i.mnemonic, suffix) } else { i.mnemonic.clone() }, i.cycles.first().unwrap(), if i.bytes > 1 {format!(", {}", i.bytes - 1)} else {"".to_owned()});
    }
}