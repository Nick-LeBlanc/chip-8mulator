mod chip;
mod instructions;
mod tests;

use chip::Chip8;
use instructions::Instructions;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

fn main() {
    let cycles = 8;
    let filename = "roms/3-corax+.ch8".to_string();
    let mut chip = Chip8::new(cycles);
    chip.load_rom(filename);

    let my_options = WindowOptions{
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X16,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    };
    let mut window = Window::new(
        "Chip 8mulator - ESC to exit",
        64,
        32,
        my_options,
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~30 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip.cycle();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(chip.display(), 64, 32)
            .unwrap();
    }
}
