use crate::instructions::Instructions;
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
    opcode:u16
}

impl Chip8 {
    pub fn new(cycles: u16) -> Self {
        let mut init_chip = Chip8 {
            memory: [0x000; 4096],
            display: [0x000u32; 64 * 32],
            program_counter: 0x200,
            index_register: 0x0,
            stack: LinkedList::new(),
            delay_timer: 0x000,
            sound_timer: 0x000,
            variable_registers: [0x000; 16],
            opcode: 0x000,
        };
        init_chip.load_font();
        return init_chip;
    }

    pub fn memory(&self) -> [u8; 4096] {
        self.memory
    }
    pub fn display(&self) -> &[u32; 64 * 32] {
        &self.display
    }
    pub fn program_counter(&self) -> &u16 {
        &self.program_counter
    }
    pub fn opcode(&self) -> &u16 {
        &self.opcode
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
        self.ins_00E0();
        self.ins_DXYN();
    }

    pub fn cycle(&mut self){
        self.opcode = (self.memory[self.program_counter as usize] as u16) << 8
            | (self.memory[(self.program_counter +1) as usize] as u16);

        self.program_counter = self.program_counter + 2;

        self.decode();

        if self.delay_timer > 0{ self.delay_timer = self.delay_timer - 1}
        if self.sound_timer > 0{ self.sound_timer = self.sound_timer - 1}
    }

    fn decode(&mut self){
        let ins = self.opcode & 0xF000;
        let data = self.opcode & 0x0FFF;
        match ins {
            0x0000 => self.ins_00E0(),
            0x1000 => self.ins_1NNN(),
            0xA000 => self.ins_ANNN(),
            0x6000 => self.ins_6XNN(),
            0x7000 => self.ins_7XNN(),
            0xD000 => self.ins_DXYN(),
            _ => self.ins_NULL(),
        }
    }
}
#[allow(dead_code)]
impl Instructions for Chip8 {
    fn ins_NULL(&mut self) {
        return
    }
    fn ins_00E0(&mut self) {
        let screen_size = self.display.len();
        self.display[0..screen_size].copy_from_slice(&vec![0x000u32; screen_size]);
    }

    fn ins_1NNN(&mut self) {
        self.program_counter = self.opcode & 0x0FFF;
    }

    fn ins_6XNN(&mut self) {
        let vx:u16 = (self.opcode & 0x0F00) >> 8u8;
        let data:u8 = (self.opcode & 0x0FF) as u8;
        self.variable_registers[vx as usize] = data;
    }
    fn ins_7XNN(&mut self) {
        let vx:u16 = (self.opcode & 0x0F00) >> 8u8;
        let data:u8 = (self.opcode & 0x0FF) as u8;
        self.variable_registers[vx as usize] = self.variable_registers[vx as usize] + data;

    }
    fn ins_ANNN(&mut self) {
        self.index_register = self.opcode & 0x0FFF;
    }

    fn ins_DXYN(&mut self){
        self.variable_registers[0xF] = 0;
        let vx:u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy:u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        let x_coord:u16 = (self.variable_registers[vx as usize] % 64) as u16;
        let y_coord:u16 = (self.variable_registers[vy as usize] % 32) as u16;

        let height = self.opcode & 0x000F;

        for row in 0..height {
            let sprite_data = self.memory[(self.index_register+row as u16) as usize];
            for col in 0..8u16{
                let pixel_data = sprite_data & (0x80 >> col);
                if pixel_data != 0{
                    let i:u16 = (y_coord + row) * 64 + (x_coord + col);
                    if self.display[i as usize] == 1 {
                        self.variable_registers[0xF] = 1;
                    }
                    self.display[i as usize] ^= 0xFFFFFFFF;
                }
            }
        }

    }
}
