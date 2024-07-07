#[cfg(test)]
mod tests {
    use super::*;
    use crate::chip::Chip8;
    use crate::instructions::Instructions;

    #[test]
    fn ins_00e0_test() {
        let mut chip = Chip8::new(8);
        chip.ins_00E0();
        assert_eq!(&[0x000u32; 64 * 32], chip.display());
    }

    #[test]
    fn ins_1nnn_test() {
        let mut chip = Chip8::new(8);
        chip.ins_1NNN();
        assert_eq!(chip.opcode(),chip.program_counter());
    }

    #[test]
    fn cycle_test(){
        let mut chip = Chip8::new(8);
        chip.load_rom("IBMLogo.ch8".to_string());
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
        let ins:u16 = 0xd5Ef;
        //Drawing row: 23, column: 52
        print!("{:#04x} ", (ins & 0x0F00) >> 8u8);
        print!("{:#04x} ", (ins & 0x00F0) >> 4u8 );
        print!("{} ", 0x80 );
        assert!(true)
    }

}