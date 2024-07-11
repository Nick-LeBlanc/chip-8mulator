#[allow(dead_code)]
pub trait Instructions {
    fn ins_null(&mut self) {}
    ///Clear Screen
    fn ins_00e0(&mut self);
    ///Jump
    fn ins_1nnn(&mut self) {}
    ///Set Register VX
    fn ins_6xnn(&mut self) {}
    ///Add value to Register VX
    fn ins_7xnn(&mut self) {}
    ///Set Index Register
    fn ins_annn(&mut self) {}
    ///Draw to screen
    fn ins_dxyn(&mut self) {}

    ///Subroutine
    fn ins_2nnn(&mut self) {}
    ///Subroutine
    fn ins_00ee(&mut self) {}

    ///Conditionally Skip if Vx value == NN
    fn ins_3xnn(&mut self) {}
    ///Conditionally Skip if Vx value != NN
    fn ins_4xnn(&mut self) {}
    ///Conditionally Skip if Vx value == Vy value
    fn ins_5xy0(&mut self) {}
    ///Conditionally Skip if Vx value != Vy value
    fn ins_9xy0(&mut self) {}

    ///Set Vx to Vy
    fn ins_8xy0(&mut self) {}

    ///Binary OR
    fn ins_8xy1(&mut self) {}

    ///Binary AND
    fn ins_8xy2(&mut self) {}

    ///Logical XOR
    fn ins_8xy3(&mut self) {}

    ///Add
    fn ins_8xy4(&mut self) {}

    ///Subtract(Vx-Vy)
    fn ins_8xy5(&mut self) {}

    ///Ambiguous
    fn ins_8xye(&mut self) {}

    ///Ambiguous
    fn ins_8xy6(&mut self) {}

    ///Subtract(Vy-Vx)
    fn ins_8xy7(&mut self) {}

    ///Jump with Offset(Ambiguous)
    fn ins_bnnn(&mut self) {}

    ///Random
    fn ins_cxnn(&mut self) {}

    ///Skip if key
    fn ins_ex9e(&mut self) {}

    ///Skip if key
    fn ins_exa1(&mut self) {}

    ///Sets Vx to the current value of the delay timer
    fn ins_fx07(&mut self) {}

    /// Sets the delay timer to the current value in Vx
    fn ins_fx15(&mut self) {}

    /// Sets the sound timer to the value in Vx
    fn ins_fx18(&mut self) {}

    ///Index register will get the value uin Vx added to it
    fn ins_fx1e(&mut self) {}

    ///Get key
    fn ins_fx0a(&mut self) {}

    ///Set Font Character
    fn ins_fx29(&mut self) {}

    ///Binary-coded decimal conversion
    fn ins_fx33(&mut self) {}

    ///Store and Load memory
    fn ins_fx55(&mut self) {}

    ///Store and Load memory(Ambiguous)
    fn ins_fx65(&mut self) {}

}
