use std::{fs::File, io::Read};

pub fn load_rom(file_path: &str) -> Vec<u8> {
    /* let rom_data = load_file(file_path);
    let rom_size = rom_data.len();
    let mut mapped_memory = vec![0u8; 0x10000];

    mapped_memory[0x0..rom_size].copy_from_slice(&rom_data); */

    load_file(file_path)
}

fn load_file(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Couldn't load roam!");
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data)
        .expect("Couldn't read roam file!");
    rom_data
}
