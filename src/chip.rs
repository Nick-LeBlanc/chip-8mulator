use crate::instructions::Instructions;
use std::borrow::Borrow;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Chip8 {
    memory: [u8; 4096],
    display: [u32; 64 * 32],
    program_counter: u16,
    index_register: u16,
    stack: LinkedList<u16>,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 16],
}

impl Chip8 {
    pub fn new(cycles: u16) -> Self {
        let mut init_chip = Chip8 {
            memory: [0x00; 4096],
            display: [0x00; 64 * 32],
            program_counter: 0x200,
            index_register: 0x00,
            stack: LinkedList::new(),
            delay_timer: 0x00,
            sound_timer: 0x00,
            variable_registers: [0x00; 16],
        };
        init_chip.load_font();
        return init_chip;
    }
    fn load_font(&mut self) {
        let font = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        self.memory[0x050..=0x09F].copy_from_slice(&font);
    }

    pub fn load_rom(&mut self, filename: String) {
        let rom: File =
            File::open(&filename).expect(format!("Could not open file: {filename}\n").as_str());
        let mut reader = BufReader::new(rom);
        let mut buffer = Vec::new();

        reader
            .read_to_end(&mut buffer)
            .expect("Could not read file properly.");

        if buffer.len() == 0 {
            panic!("File has no data\n")
        }
        for i in 0..buffer.len() {
            self.memory[0x200 + i] = buffer[i];
        }
    }

    pub fn draw_test(&mut self) {
        self.display[0] = 0x020;
    }
}
#[allow(dead_code)]
impl Instructions for Chip8 {
    fn memory(&self) -> [u8; 4096] {
        self.memory
    }
    fn display(&self) -> &[u32; 64 * 32] {
        &self.display
    }
    fn program_counter(&self) -> u16 {
        self.program_counter
    }
    fn index_register(&self) -> u16 {
        self.index_register
    }
    fn stack(&self) -> &LinkedList<u16> {
        &self.stack
    }
    fn delay_timer(&self) -> u8 {
        self.delay_timer
    }
    fn sound_timer(&self) -> u8 {
        self.sound_timer
    }
    fn variable_registers(&self) -> [u8; 16] {
        self.variable_registers
    }

    fn ins_00E0(&mut self) {
        let screen_size = self.display.len();
        self.display[0..screen_size].copy_from_slice(&vec![0x000; screen_size]);
    }
}
