mod chip;
mod instructions;
mod tests;

use chip::Chip8;
use instructions::Instructions;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::env;
use std::time::SystemTime;

fn main() {
    print!("hello world!");
    let inputs = handle_input(env::args().collect());

    let mut chip = Chip8::new();
    chip.load_rom(inputs.0);
    let cycles = inputs.1;

    let my_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X16,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    };
    let mut window =
        Window::new("Chip 8mulator - ESC to exit", 64, 32, my_options).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~30 fps update rate
    window.set_target_fps(180);

    let mut prev_cycle = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip.get_input(set_controls(&window));

        let curr_cycle = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let delta_time = curr_cycle - prev_cycle;
        if delta_time > cycles {
            chip.cycle();
            prev_cycle = curr_cycle;
        }
        window.update_with_buffer(chip.display(), 64, 32).unwrap();
    }
}
fn set_controls(window: &Window) -> [u8; 16] {
    let mut output: [u8; 16] = [0x0; 16];
    output[0x1] = if window.is_key_down(Key::Key1) { 1 } else { 0 };
    output[0x2] = if window.is_key_down(Key::Key2) { 1 } else { 0 };
    output[0x3] = if window.is_key_down(Key::Key3) { 1 } else { 0 };
    output[0xC] = if window.is_key_down(Key::Key4) { 1 } else { 0 };
    output[0x4] = if window.is_key_down(Key::Q) { 1 } else { 0 };
    output[0x5] = if window.is_key_down(Key::W) { 1 } else { 0 };
    output[0x6] = if window.is_key_down(Key::E) { 1 } else { 0 };
    output[0xD] = if window.is_key_down(Key::R) { 1 } else { 0 };
    output[0x7] = if window.is_key_down(Key::A) { 1 } else { 0 };
    output[0x8] = if window.is_key_down(Key::S) { 1 } else { 0 };
    output[0x9] = if window.is_key_down(Key::D) { 1 } else { 0 };
    output[0xE] = if window.is_key_down(Key::F) { 1 } else { 0 };
    output[0xA] = if window.is_key_down(Key::Z) { 1 } else { 0 };
    output[0x0] = if window.is_key_down(Key::X) { 1 } else { 0 };
    output[0xB] = if window.is_key_down(Key::C) { 1 } else { 0 };
    output[0xF] = if window.is_key_down(Key::V) { 1 } else { 0 };

    output
}

fn handle_input(args: Vec<String>) -> (String, u128) {
    if args.len() != 3 {
        panic!("Error: Wrong number of Arguments \ncargo run <Rom> <Cycles>");
    }
    let cycles = args[2].parse::<u128>().unwrap_or(4);
    let filename = args[1].to_string();

    return (filename, cycles);
}
