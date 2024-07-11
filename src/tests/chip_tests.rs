#[cfg(test)]
mod tests {
    use super::*;
    use crate::chip::Chip8;
    use crate::instructions::Instructions;

    #[test]
    fn ins_00e0_test() {
        let mut chip = Chip8::new(8);
        chip.ins_00e0();
        assert_eq!(&[0x000u32; 64 * 32], chip.display());
    }

    #[test]
    fn ins_1nnn_test() {
        let mut chip = Chip8::new(8);
        chip.ins_1nnn();
        assert_eq!(chip.opcode(),chip.program_counter());
    }

    #[test]
    fn cycle_test(){
        let mut chip = Chip8::new(8);
        chip.load_rom("roms/test_opcode.ch8".to_string());
        assert_ne!(0x000,chip.memory()[0x200+1]);
        for i in 0..100{
            chip.cycle();
        }
        assert!(true)
    }

    #[test]
    fn print_rom(){
        let mut chip = Chip8::new(8);
        chip.load_rom("test_opcode.ch8".to_string());
        assert_ne!(0x000,chip.memory()[0x200+1]);
        //self.opcode = self.memory[(self.program_counter + 1) as usize] as u16;
        for i in 0x200..0x600{
            print!("{:#04x} ", chip.memory()[i]);
            if i % 5 == 0 {println!()}
        }
        assert!(true)
    }

    #[test]
    fn bit_test(){
        let data = 0x1F;
        let ones = data % 10;
        let tens = ((data % 100) - ones) / 10;
        let hundreds = (data - (data % 100)) / 100;
        println!("{}", data);
        println!("{}, {}, {}", hundreds, tens, ones);
    }

    #[test]
    fn compare_test(){
        let ins:u16 = 0xA202 & 0x0FFF;
        println!("old: {:#04x}", -119);
    }

}