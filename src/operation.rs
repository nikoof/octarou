#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Operation {
    ClearScreen,
}

impl Operation {
    pub fn new(opcode: u16) -> Self {
        match opcode {
            _ => Self::ClearScreen,
        }
    }
}
