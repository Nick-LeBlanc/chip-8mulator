use std::collections::LinkedList;

struct Chip8 {
    memory: [u8; 4096],
    display: [u32; 64 * 32],
    program_counter: u16,
    index_register: u16,
    stack: LinkedList<u16>,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 16],
}

fn main() {
    println!();
}
