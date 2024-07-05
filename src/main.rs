mod chip;
mod instructions;
use chip::Chip8;
use instructions::Instructions;

fn main() {
    let cycles = 8;
    let filename = "IBMLogo.ch8".to_string();
    let mut chip = Chip8::new(cycles);
    chip.load_rom(filename);


    println!("{:#05x}",0x0e0 & 0xFFF);
}
