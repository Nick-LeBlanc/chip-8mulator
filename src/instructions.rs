use std::collections::LinkedList;
#[allow(dead_code)]
pub trait Instructions {
    fn ins_NULL(&mut self) {}
    ///Clear Screen
    fn ins_00E0(&mut self);
    ///Jump
    fn ins_1NNN(&mut self) {}
    ///Set Register VX
    fn ins_6XNN(&mut self) {}
    ///Add value to Register VX
    fn ins_7XNN(&mut self) {}
    ///Set Index Register
    fn ins_ANNN(&mut self) {}
    ///Draw to screen
    fn ins_DXYN(&mut self) {}

}
