use std::collections::LinkedList;
mod chip;
use chip::Chip8;

fn main() {
    let cycles = 8;
    let filename = "IBMLogo.ch8".to_string();
    let mut chip = Chip8::new(cycles);
    chip.load_rom(filename);

    for i in 0..100 {
        print!("{:#05x} ", chip.memory[0x200 + i]);
        if (i + 1) % 5 == 0 {
            println!()
        };
    }
}
