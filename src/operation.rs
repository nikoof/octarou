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
    pub fn new(opcode: u16) -> Self {
        use Operation::*;

        let first_nibble = opcode & 0xF000;
        match first_nibble {
            0x0000 => match opcode {
                0x00E0 => ClearScreen,
                0x00EE => Return,
                _ => unimplemented!(),
            },
            0x1000 => {
                let address = nnn(opcode);
                Jump { address }
            }
            0x3000 => {
                let (x, value) = xnn(opcode);
                SkipEqLiteral { x, value }
            }
            0x4000 => {
                let (x, value) = xnn(opcode);
                SkipNotEqLiteral { x, value }
            }
            0x5000 => {
                let (x, y, _) = xyn(opcode);
                SkipEq { x, y }
            }
            0x9000 => {
                let (x, y, _) = xyn(opcode);
                SkipNotEq { x, y }
            }
            0x6000 => {
                let (destination, value) = xnn(opcode);
                SetLiteral { destination, value }
            }
            0x7000 => {
                let (destination, value) = xnn(opcode);
                AddLiteral { destination, value }
            }
            0x8000 => {
                let (lhs, rhs, op) = xyn(opcode);
                match op {
                    0 => Set {
                        destination: lhs,
                        source: rhs,
                    },
                    0x1 => Or { lhs, rhs },
                    0x2 => And { lhs, rhs },
                    0x3 => Xor { lhs, rhs },
                    0x4 => Add { lhs, rhs },
                    0x5 => Sub { lhs, rhs },
                    0x7 => Sub { lhs: rhs, rhs: lhs },
                    0x6 => LeftShift { lhs, rhs },
                    0xE => RightShift { lhs, rhs },
                    _ => unimplemented!(),
                }
            }
            0xA000 => SetIndex {
                source: nnn(opcode),
            },

            // TODO: Make this op's behaviour configurable.
            0xB000 => JumpOffset {
                address: nnn(opcode),
                offset_register: 0,
            },

            0xC000 => {
                let (x, mask) = xnn(opcode);
                Random { x, mask }
            }

            0xD000 => {
                let (x, y, value) = xyn(opcode);
                Draw { x, y, value }
            }

            0xE000 => {
                let (key_register, op) = xnn(opcode);
                match op {
                    0x9E => SkipIfKey { key_register },
                    0xA1 => SkipIfNotKey { key_register },
                    _ => unimplemented!(),
                }
            }

            0xF000 => {
                let (x, nn) = xnn(opcode);
                match nn {
                    0x07 => GetDelay { destination: x },
                    0x15 => SetDelay { source: x },
                    0x18 => SetSound { source: x },
                    0x1E => AddIndex { source: x },
                    0x0A => GetKey { destination: x },
                    0x29 => SetIndexFont { source: x },
                    0x33 => DecimalConversion { source: x },
                    0x55 => StoreMemory { registers: x },
                    0x65 => LoadMemory { registers: x },
                    _ => unimplemented!(),
                }
            }

            _ => todo!("Implement all operations"),
        }
    }
}
