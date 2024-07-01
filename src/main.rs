mod chip;
mod instructions;
use chip::Chip8;
use instructions::Instructions;

fn main() {
    let cycles = 8;
    let filename = "IBMLogo.ch8".to_string();
    let mut chip = Chip8::new(cycles);
    chip.load_rom(filename);
    chip.draw_test();
    chip.ins_00E0();
    println!("{:#05x}, {:#05x}", chip.display()[0], chip.display()[1]);
    // for i in 0x200..0xFFF {
    //     print!("{:#05x} ", chip.memory()[i]);
    //     if (i + 1) % 5 == 0 {
    //         println!();
    //     };
    // }
}
