mod chip;
mod instructions;
use chip::Chip8;
use instructions::Instructions;

fn main() {
    let cycles = 8;
    let filename = "IBMLogo.ch8".to_string();
    let mut chip = Chip8::new(cycles);
    chip.load_rom(filename);

    for i in 0x200..0xFFF {
        print!("{:#05x} ", chip.memory()[i]);
        if (i + 1) % 5 == 0 {
            println!();
        };
    }
}
