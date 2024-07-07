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
        self.ins_00e0();
        self.ins_dxyn();
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
            0x0000 => self.ins_00e0(),
            0x1000 => self.ins_1nnn(),
            0xA000 => self.ins_annn(),
            0x6000 => self.ins_6xnn(),
            0x7000 => self.ins_7xnn(),
            0xD000 => self.ins_dxyn(),
             _ => self.ins_null(),
        }
    }
}
#[allow(dead_code)]
impl Instructions for Chip8 {
    fn ins_null(&mut self) {
        return
    }
    fn ins_00e0(&mut self) {
        let screen_size = self.display.len();
        self.display[0..screen_size].copy_from_slice(&vec![0x000u32; screen_size]);
    }

    fn ins_1nnn(&mut self) {
        self.program_counter = self.opcode & 0x0FFF;
    }

    fn ins_6xnn(&mut self) {
        let vx:u16 = (self.opcode & 0x0F00) >> 8u8;
        let data:u8 = (self.opcode & 0x0FF) as u8;
        self.variable_registers[vx as usize] = data;
    }
    fn ins_7xnn(&mut self) {
        let vx:u16 = (self.opcode & 0x0F00) >> 8u8;
        let data:u16 = (self.opcode & 0x00FF);
        self.variable_registers[vx as usize] = (self.variable_registers[vx as usize] as u16 + data) as u8;

    }
    fn ins_annn(&mut self) {
        self.index_register = self.opcode & 0x0FFF;
    }

    fn ins_dxyn(&mut self){
        self.variable_registers[0xF] = 0;
        let vx:u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy:u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        let x_coord:u16 = (self.variable_registers[vx as usize] % 64) as u16;
        let y_coord:u16 = (self.variable_registers[vy as usize] % 32) as u16;

        let height = self.opcode & 0x000F;

        for row in 0..height {
            let sprite_data = self.memory[(self.index_register+row) as usize];
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

    fn ins_2nnn(&mut self) {
        let mem_loc = self.opcode & 0x0FFF;
        self.program_counter = mem_loc;
        self.stack.push_front(mem_loc);
    }

    fn ins_00ee(&mut self) {
        self.program_counter = self.stack.pop_front().unwrap();
    }

    fn ins_3xnn(&mut self) {}

    fn ins_4xnn(&mut self) {}

    fn ins_5xy0(&mut self) {}

    fn ins_9xy0(&mut self) {}


    fn ins_8xy0(&mut self) {}


    fn ins_8xy1(&mut self) {}


    fn ins_8xy2(&mut self) {}


    fn ins_8xy3(&mut self) {}


    fn ins_8xy4(&mut self) {}


    fn ins_8xy5(&mut self) {}


    fn ins_8xye(&mut self) {}


    fn ins_8xy6(&mut self) {}


    fn ins_8xy7(&mut self) {}


    fn ins_bnnn(&mut self) {}


    fn ins_cxnn(&mut self) {}


    fn ins_ex9e(&mut self) {}


    fn ins_exa1(&mut self) {}


    fn ins_fx07(&mut self) {}


    fn ins_fx15(&mut self) {}


    fn ins_fx18(&mut self) {}


    fn ins_fx1e(&mut self) {}


    fn ins_fx0a(&mut self) {}


    fn ins_fx29(&mut self) {}


    fn ins_fx33(&mut self) {}


    fn ins_fx55(&mut self) {}


    fn ins_fx65(&mut self) {}

}
