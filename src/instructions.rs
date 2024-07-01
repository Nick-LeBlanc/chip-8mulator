use std::collections::LinkedList;
#[allow(dead_code)]
pub trait Instructions {
    fn memory(&self) -> [u8; 4096];
    fn display(&self) -> &[u32; 64 * 32];
    fn program_counter(&self) -> u16;
    fn index_register(&self) -> u16;
    fn stack(&self) -> &LinkedList<u16>;
    fn delay_timer(&self) -> u8;
    fn sound_timer(&self) -> u8;
    fn variable_registers(&self) -> [u8; 16];

    ///Clear Screen
    fn ins_00E0(&mut self);
    ///Jump
    fn ins_1NNN() {}
    ///Set Register VX
    fn ins_6XNN() {}
    ///Add value to Register VX
    fn ins_7XNN() {}
    ///Set Index Register
    fn ins_ANNN() {}
    ///Draw to screen
    fn ins_DXYN() {}
}
