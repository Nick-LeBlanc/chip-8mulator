use crate::instructions::Instructions;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use rand::prelude::*;

pub struct Chip8 {
    memory: [u8; 4096],
    display: [u32; 64 * 32],
    program_counter: u16,
    index_register: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8;16],
    variable_registers: [u8; 16],
    opcode:u16
}

impl Chip8 {
    pub fn new() -> Self {
        let mut init_chip = Chip8 {
            memory: [0x000; 4096],
            display: [0x000u32; 64 * 32],
            program_counter: 0x200,
            index_register: 0x0,
            stack: Vec::with_capacity(16),
            delay_timer: 0x000,
            sound_timer: 0x000,
            keypad:[0x000; 16],
            variable_registers: [0x000; 16],
            opcode: 0x000
        };
        init_chip.load_font();
        return init_chip;
    }

    pub fn display(&self) -> &[u32; 64 * 32] {
        &self.display
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
    pub fn get_input(&mut self, inputs:[u8;16]){
        self.keypad.copy_from_slice(&inputs)
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

        match ins {
            0x0000 => {
                let last_bit = self.opcode & 0x000F;
                match last_bit{
                    0x0 => self.ins_00e0(),
                    0xE => self.ins_00ee(),
                    _   => self.ins_null()
                }
            },
            0x1000 => self.ins_1nnn(),
            0x2000 => self.ins_2nnn(),
            0x3000 => self.ins_3xnn(),
            0x4000 => self.ins_4xnn(),
            0x5000 => self.ins_5xy0(),
            0x6000 => self.ins_6xnn(),
            0x7000 => self.ins_7xnn(),
            0x8000 => {
                let last_bit = self.opcode & 0x000F;
                match last_bit{
                    0x0 => self.ins_8xy0(),
                    0x1 => self.ins_8xy1(),
                    0x2 => self.ins_8xy2(),
                    0x3 => self.ins_8xy3(),
                    0x4 => self.ins_8xy4(),
                    0x5 => self.ins_8xy5(),
                    0x6 => self.ins_8xy6(),
                    0x7 => self.ins_8xy7(),
                    0xE => self.ins_8xye(),
                    _   => self.ins_null()
                }
            },
            0x9000 => self.ins_9xy0(),
            0xA000 => self.ins_annn(),
            0xB000 => self.ins_bnnn(),
            0xC000 => self.ins_cxnn(),
            0xD000 => self.ins_dxyn(),
            0xE000 => {
                let last_bit = self.opcode & 0x00FF;
                match last_bit{
                    0xa1 => self.ins_exa1(),
                    0x9e => self.ins_ex9e(),
                    _   => self.ins_null()
                }
            },
            0xF000 => {
                let last_bit = self.opcode & 0x00FF;
                match last_bit{
                    0x07 => self.ins_fx07(),
                    0x0A => self.ins_fx0a(),
                    0x15 => self.ins_fx18(),
                    0x18 => self.ins_fx18(),
                    0x1E => self.ins_fx1e(),
                    0x29 => self.ins_fx29(),
                    0x33 => self.ins_fx33(),
                    0x55 => self.ins_fx55(),
                    0x65 => self.ins_fx65(),
                    _   => self.ins_null()
                }
            },
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
        let data:u16 = self.opcode & 0x00FF;
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
                    let mut i:u16 = x_coord + col + ((y_coord + row) * 64);
                    if i > self.display.len() as u16 {i = self.display.len() as u16 - 1}
                    if self.display[i as usize] == 0xFFFFFFFF {
                        self.variable_registers[0xF] = 1;
                    }
                    self.display[i as usize] ^= 0xFFFFFFFF;
                }
            }
        }

    }

    fn ins_2nnn(&mut self) {
        let mem_loc = self.opcode & 0x0FFF;
        self.stack.push(self.program_counter);
        self.program_counter = mem_loc;
    }

    fn ins_00ee(&mut self) {
        self.program_counter = self.stack.pop().unwrap();
    }

    fn ins_3xnn(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let data:u8 = (self.opcode & 0x00FF) as u8;
        if self.variable_registers[vx as usize] == data {
            self.program_counter = self.program_counter + 2;
        }
    }

    fn ins_4xnn(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let data:u8 = (self.opcode & 0x00FF) as u8;
        if self.variable_registers[vx as usize] != data {
            self.program_counter = self.program_counter + 2;
        }
    }

    fn ins_5xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;
        if self.variable_registers[vx as usize] == self.variable_registers[vy as usize] {
            self.program_counter = self.program_counter + 2;
        }
    }

    fn ins_9xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;
        if self.variable_registers[vx as usize] != self.variable_registers[vy as usize] {
            self.program_counter = self.program_counter + 2;
        }
    }


    fn ins_8xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;
        self.variable_registers[vx as usize] = self.variable_registers[vy as usize]
    }


    fn ins_8xy1(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;
        self.variable_registers[vx as usize] = self.variable_registers[vx as usize] | self.variable_registers[vy as usize]
    }


    fn ins_8xy2(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;
        self.variable_registers[vx as usize] = self.variable_registers[vx as usize] & self.variable_registers[vy as usize]
    }


    fn ins_8xy3(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;
        self.variable_registers[vx as usize] = self.variable_registers[vx as usize] ^ self.variable_registers[vy as usize]
    }


    fn ins_8xy4(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;
        let data = u8::overflowing_add(self.variable_registers[vx as usize],
                                       self.variable_registers[vy as usize]);
        self.variable_registers[vx as usize] =data.0;
        self.variable_registers[0xF] = if data.1 {1} else {0};
    }


    //If Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    fn ins_8xy5(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;

        let data = u8::overflowing_sub(self.variable_registers[vx as usize],
                                       self.variable_registers[vy as usize]);

        self.variable_registers[vx as usize] =data.0;
        self.variable_registers[0xF] = if data.1 {0} else {1};
    }

    fn ins_8xy7(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let vy= (self.opcode & 0x00F0) >> 4u8;

        let data = u8::overflowing_sub(self.variable_registers[vy as usize],
                                       self.variable_registers[vx as usize]);

        self.variable_registers[vx as usize] =data.0;
        self.variable_registers[0xF] = if data.1 {0} else {1};
    }

    fn ins_8xye(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let data = self.variable_registers[vx as usize] << 1u8;

        self.variable_registers[0xF] = (self.variable_registers[vx as usize] & 0x80u8) >> 7u8;

        self.variable_registers[vx as usize] =data;
    }


    fn ins_8xy6(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let data = self.variable_registers[vx as usize] >> 1u8;

        self.variable_registers[0xF] = (self.variable_registers[vx as usize] & 0x8u8) >> 7u8;

        self.variable_registers[vx as usize] =data;
    }


    fn ins_bnnn(&mut self) {
        self.index_register = (self.opcode & 0x0FFF) + (self.variable_registers[0x0]) as u16
    }


    fn ins_cxnn(&mut self) {
        let rand_number = random::<u16>();
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let data = self.opcode & 0x00FF;
        self.variable_registers[vx as usize] = (data & rand_number) as u8;
    }


    fn ins_ex9e(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let key = self.variable_registers[vx as usize];
        if self.keypad[key as usize] != 0{
            self.program_counter = self.program_counter + 2;
        }
    }


    fn ins_exa1(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        let key = self.variable_registers[vx as usize];
        if self.keypad[key as usize] == 0 {
            self.program_counter = self.program_counter + 2;
        }
    }


    fn ins_fx07(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        self.variable_registers[vx as usize] = self.delay_timer;
    }


    fn ins_fx15(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        self.delay_timer = self.variable_registers[vx as usize];
    }


    fn ins_fx18(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        self.sound_timer = self.variable_registers[vx as usize];
    }


    fn ins_fx1e(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        self.index_register = self.index_register + self.variable_registers[vx as usize] as u16;
    }


    fn ins_fx0a(&mut self) {
        self.program_counter = self.program_counter - 2;
    }


    fn ins_fx29(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        self.index_register = 0x050 + (self.variable_registers[vx as usize] * 5) as u16;
    }


    fn ins_fx33(&mut self) {

        let vx = (self.opcode & 0x0F00) >> 8u8;
        let data = self.variable_registers[vx as usize];
        let pos = self.index_register;
        let ones = data % 10;
        let tens = ((data % 100) - ones) / 10;
        let hundreds = (data - (data % 100)) / 100;
        self.memory[pos as usize] = hundreds;
        self.memory[(pos + 1) as usize] = tens;
        self.memory[(pos + 2) as usize] = ones;
    }


    fn ins_fx55(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        if vx == 0x0 {
            self.memory[self.index_register as usize] = self.variable_registers[0x0];
            return;
        }else{
            self.memory[self.index_register as usize..=(self.index_register + vx) as usize].
                copy_from_slice(&self.variable_registers[0x0..vx as usize + 1]);
        }
    }


    fn ins_fx65(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8u8;
        if vx == 0x0 {
            self.variable_registers[0x0] = self.memory[self.index_register as usize];
            return;
        }else{
            self.variable_registers[0x0..=vx as usize]
                .copy_from_slice(&self.memory[self.index_register as usize..(self.index_register + vx) as usize + 1]);
        }

    }

}
