#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Instruction {
    // Chip8 instructions
    ClearScreen,
    Draw {
        x: usize,
        y: usize,
        sprite_height: usize,
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
        dest: usize,
        src: usize,
    },

    SetLiteral {
        dest: usize,
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
        dest: usize,
        value: u8,
    },

    Sub {
        lhs: usize,
        rhs: usize,
        dest: usize,
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
        src: usize,
    },

    AddIndex {
        src: usize,
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
        dest: usize,
    },

    SetDelay {
        src: usize,
    },

    SetSound {
        src: usize,
    },

    GetKey {
        dest: usize,
    },

    SetIndexFont {
        src: usize,
        big: bool,
    },

    DecimalConversion {
        src: usize,
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

    // Schip extension
    Hires,
    Lores,

    ScrollRight,
    ScrollLeft,
    ScrollDown {
        amount: usize,
    },

    SaveFlags {
        x: usize,
    },
    LoadFlags {
        x: usize,
    },

    Exit,
}

impl Instruction {
    pub fn new(opcode: u16) -> Option<Self> {
        use Instruction::*;

        let first_nibble = opcode & 0xF000;
        match first_nibble {
            0x0000 => match opcode {
                0x00E0 => Some(ClearScreen),
                0x00EE => Some(Return),
                0x00FF => Some(Hires),
                0x00FE => Some(Lores),
                0x00FB => Some(ScrollRight),
                0x00FC => Some(ScrollLeft),
                0x00FD => Some(Exit),
                _ => match opcode & 0x00F0 {
                    0x00C0 => Some(ScrollDown {
                        amount: xyn(opcode).2 as usize,
                    }),
                    _ => None,
                },
            },
            0x1000 => {
                let address = nnn(opcode);
                Some(Jump { address })
            }
            0x2000 => Some(Call {
                address: nnn(opcode),
            }),
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
                let (dest, value) = xnn(opcode);
                Some(SetLiteral { dest, value })
            }
            0x7000 => {
                let (dest, value) = xnn(opcode);
                Some(AddLiteral { dest, value })
            }
            0x8000 => {
                let (lhs, rhs, op) = xyn(opcode);
                match op {
                    0x0 => Some(Set {
                        dest: lhs,
                        src: rhs,
                    }),
                    0x1 => Some(Or { lhs, rhs }),
                    0x2 => Some(And { lhs, rhs }),
                    0x3 => Some(Xor { lhs, rhs }),
                    0x4 => Some(Add { lhs, rhs }),
                    0x5 => Some(Sub {
                        lhs,
                        rhs,
                        dest: lhs,
                    }),
                    0x7 => Some(Sub {
                        lhs: rhs,
                        rhs: lhs,
                        dest: lhs,
                    }),
                    0x6 => Some(RightShift { lhs, rhs }),
                    0xE => Some(LeftShift { lhs, rhs }),
                    _ => None,
                }
            }
            0xA000 => Some(SetIndex { src: nnn(opcode) }),
            0xB000 => Some(JumpOffset {
                address: nnn(opcode),
                offset_register: xnn(opcode).0,
            }),

            0xC000 => {
                let (x, mask) = xnn(opcode);
                Some(Random { x, mask })
            }

            0xD000 => {
                let (x, y, sprite_height) = xyn(opcode);
                Some(Draw {
                    x,
                    y,
                    sprite_height: sprite_height as usize,
                })
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
                    0x07 => Some(GetDelay { dest: x }),
                    0x15 => Some(SetDelay { src: x }),
                    0x18 => Some(SetSound { src: x }),
                    0x1E => Some(AddIndex { src: x }),
                    0x0A => Some(GetKey { dest: x }),
                    0x29 => Some(SetIndexFont { src: x, big: false }),
                    0x30 => Some(SetIndexFont { src: x, big: true }),
                    0x33 => Some(DecimalConversion { src: x }),
                    0x55 => Some(StoreMemory { registers: x }),
                    0x65 => Some(LoadMemory { registers: x }),
                    0x75 => Some(SaveFlags { x }),
                    0x85 => Some(LoadFlags { x }),
                    _ => None,
                }
            }

            _ => None,
        }
    }
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
