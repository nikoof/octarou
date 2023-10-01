#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Operation {
    ClearScreen,
    Draw {
        x: usize,
        y: usize,
        value: u8,
    },

    Jump {
        address: usize,
    },

    JumpOffset {
        address: usize,
        offset_register: usize,
    },

    Call {
        address: usize,
    },
    Return,

    Set {
        destination: usize,
        source: usize,
    },

    SetLiteral {
        destination: usize,
        value: u8,
    },

    Or {
        lhs: usize,
        rhs: usize,
    },

    And {
        lhs: usize,
        rhs: usize,
    },

    Xor {
        lhs: usize,
        rhs: usize,
    },

    Add {
        lhs: usize,
        rhs: usize,
    },

    AddLiteral {
        destination: usize,
        value: u8,
    },

    Sub {
        lhs: usize,
        rhs: usize,
    },

    LeftShift {
        lhs: usize,
        rhs: usize,
    },

    RightShift {
        lhs: usize,
        rhs: usize,
    },

    SetIndex {
        source: usize,
    },

    AddIndex {
        source: usize,
    },

    SkipEq {
        x: usize,
        y: usize,
    },

    SkipNotEq {
        x: usize,
        y: usize,
    },

    SkipEqLiteral {
        x: usize,
        value: u8,
    },

    SkipNotEqLiteral {
        x: usize,
        value: u8,
    },

    SkipIfKey {
        key_register: usize,
    },

    SkipIfNotKey {
        key_register: usize,
    },

    GetDelay {
        destination: usize,
    },

    SetDelay {
        source: usize,
    },

    SetSound {
        source: usize,
    },

    GetKey {
        destination: usize,
    },

    SetIndexFont {
        source: usize,
    },

    DecimalConversion {
        source: usize,
    },

    StoreMemory {
        registers: usize,
    },

    LoadMemory {
        registers: usize,
    },

    Random {
        x: usize,
        mask: u8,
    },
}

fn xyn(opcode: u16) -> (usize, usize, u8) {
    (
        ((opcode & 0x0F00) >> 8) as usize,
        ((opcode & 0x00F0) >> 4) as usize,
        (opcode & 0x000F) as u8,
    )
}

fn xnn(opcode: u16) -> (usize, u8) {
    (((opcode & 0x0F00) >> 8) as usize, (opcode & 0x00FF) as u8)
}

fn nnn(opcode: u16) -> usize {
    (opcode & 0x0FFF) as usize
}

impl Operation {
    pub fn new(opcode: u16) -> Option<Self> {
        use Operation::*;

        let first_nibble = opcode & 0xF000;
        match first_nibble {
            0x0000 => match opcode {
                0x00E0 => Some(ClearScreen),
                0x00EE => Some(Return),
                _ => None,
            },
            0x1000 => {
                let address = nnn(opcode);
                Some(Jump { address })
            }
            0x3000 => {
                let (x, value) = xnn(opcode);
                Some(SkipEqLiteral { x, value })
            }
            0x4000 => {
                let (x, value) = xnn(opcode);
                Some(SkipNotEqLiteral { x, value })
            }
            0x5000 => {
                let (x, y, _) = xyn(opcode);
                Some(SkipEq { x, y })
            }
            0x9000 => {
                let (x, y, _) = xyn(opcode);
                Some(SkipNotEq { x, y })
            }
            0x6000 => {
                let (destination, value) = xnn(opcode);
                Some(SetLiteral { destination, value })
            }
            0x7000 => {
                let (destination, value) = xnn(opcode);
                Some(AddLiteral { destination, value })
            }
            0x8000 => {
                let (lhs, rhs, op) = xyn(opcode);
                match op {
                    0 => Some(Set {
                        destination: lhs,
                        source: rhs,
                    }),
                    0x1 => Some(Or { lhs, rhs }),
                    0x2 => Some(And { lhs, rhs }),
                    0x3 => Some(Xor { lhs, rhs }),
                    0x4 => Some(Add { lhs, rhs }),
                    0x5 => Some(Sub { lhs, rhs }),
                    0x7 => Some(Sub { lhs: rhs, rhs: lhs }),
                    0x6 => Some(LeftShift { lhs, rhs }),
                    0xE => Some(RightShift { lhs, rhs }),
                    _ => None,
                }
            }
            0xA000 => Some(SetIndex {
                source: nnn(opcode),
            }),

            // TODO: Make this op's behaviour configurable.
            0xB000 => Some(JumpOffset {
                address: nnn(opcode),
                offset_register: 0,
            }),

            0xC000 => {
                let (x, mask) = xnn(opcode);
                Some(Random { x, mask })
            }

            0xD000 => {
                let (x, y, value) = xyn(opcode);
                Some(Draw { x, y, value })
            }

            0xE000 => {
                let (key_register, op) = xnn(opcode);
                match op {
                    0x9E => Some(SkipIfKey { key_register }),
                    0xA1 => Some(SkipIfNotKey { key_register }),
                    _ => None,
                }
            }

            0xF000 => {
                let (x, nn) = xnn(opcode);
                match nn {
                    0x07 => Some(GetDelay { destination: x }),
                    0x15 => Some(SetDelay { source: x }),
                    0x18 => Some(SetSound { source: x }),
                    0x1E => Some(AddIndex { source: x }),
                    0x0A => Some(GetKey { destination: x }),
                    0x29 => Some(SetIndexFont { source: x }),
                    0x33 => Some(DecimalConversion { source: x }),
                    0x55 => Some(StoreMemory { registers: x }),
                    0x65 => Some(LoadMemory { registers: x }),
                    _ => None,
                }
            }

            _ => None,
        }
    }
}
