use crate::instructions::Instructions;
use std::borrow::Borrow;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const V_WIDTH: u32 = 64;
const V_HEIGHT:u32 = 32;
pub struct Chip8 {
    memory: [u8; 4096],
    display: [u32; (V_WIDTH * V_HEIGHT) as usize],
    program_counter: u16,
    index_register: u16,
    stack: LinkedList<u16>,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 16],
    opcode: u16
}

impl Chip8 {
    pub fn new(cycles: u16) -> Self {
        let mut init_chip = Chip8 {
            memory: [0x000; 4096],
            display: [0x000; (V_WIDTH * V_HEIGHT) as usize],
            program_counter: 0x200,
            index_register: 0x000,
            stack: LinkedList::new(),
            delay_timer: 0x000,
            sound_timer: 0x000,
            variable_registers: [0x000; 16],
            opcode: 0x000
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

    pub fn cycle(&mut self){
        self.opcode = self.fetch();

        self.decode();

        if self.delay_timer > 0 { self.delay_timer = self.delay_timer -1; }
        if self.sound_timer > 0 { self.sound_timer = self.sound_timer -1; }
    }

   fn fetch(&mut self) -> u16{
        let left =  (self.memory[self.program_counter as usize]) as u16;
        let right  = (self.memory[(self.program_counter +1)as usize]) as u16;
        self.program_counter = self.program_counter + 2;
        left << 8u8 | right
    }


    fn decode(&mut self){

    }
}
#[allow(dead_code)]
impl Instructions for Chip8 {


    fn ins_00E0(&mut self) {
        let screen_size = self.display.len();
        self.display[0..screen_size].copy_from_slice(&vec![0x000; screen_size]);
    }
    fn ins_1NNN(&mut self) {
        self.program_counter = self.opcode & 0xFFF;
    }
    fn ins_6XNN(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FF) as u8;
        self.variable_registers[vx as usize] = byte;
    }

    fn ins_7XNN(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FF) as u8;
        self.variable_registers[vx as usize] = self.variable_registers[vx as usize] + byte;
    }

    fn ins_ANNN(&mut self) {
        self.index_register = self.opcode & 0x0FFF
    }

    fn ins_DXYN(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x0F00) >> 4;
        let height = self.opcode & 0x000F;
        let xPos = self.variable_registers[vx as usize] % V_WIDTH as u8;
        let yPos = self.variable_registers[vy as usize] % V_HEIGHT as u8;
        self.variable_registers[0xF] = 0;

        for row in 0..height{
            let sprite = self.memory[(self.index_register + row) as usize];
            for col in 0..8{
                let pixel = sprite & (0x80 >> col);
                let mut screen_pixel = self.display[((yPos+row as u8) * V_WIDTH as u8 + (xPos +col)) as usize];

                if pixel != 0 {
                    if screen_pixel == 0xFFFFFFFF{
                        self.variable_registers[0xFusize] = 1;
                    }
                    screen_pixel ^= 0xFFFFFFFF;
                }
            }
        }
    }
}
