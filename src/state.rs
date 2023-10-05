use crate::display::{Display, HEIGHT, WIDTH};
use crate::operation::Operation;
use anyhow::Result;
use rand::random;

const FONT_ADDRESS: usize = 0x200;
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct State {
    memory: [u8; 4096],
    pc: usize,
    index: usize,
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
    variables: [u8; 16],
    display: Display,
}

impl State {
    pub fn new() -> Result<Self> {
        Ok(Self {
            memory: [0; 4096],
            pc: 0,
            index: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            variables: [0; 16],
            display: Display::new()?,
        })
    }

    pub fn default() -> Result<Self> {
        let mut memory = [0u8; 4096];
        memory[0x50..=0x9F].copy_from_slice(&FONT);

        Ok(Self {
            memory,
            ..Self::new()?
        })
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        self.memory[FONT_ADDRESS..FONT_ADDRESS + program.len()].copy_from_slice(&program);
        self.pc = FONT_ADDRESS;
    }

    pub fn should_run(&self) -> bool {
        self.display.window.is_open()
    }

    pub fn tick(&mut self) {
        let next_op = self.next_operation();
        self.execute_operation(next_op);
        self.display.update();
    }

    fn next_operation(&mut self) -> Operation {
        let opcode =
            (self.memory[self.pc] as u16).checked_shl(8).unwrap() + self.memory[self.pc + 1] as u16;
        self.pc += 2;
        if let Some(op) = Operation::new(opcode) {
            op
        } else {
            self.next_operation()
        }
    }

    fn execute_operation(&mut self, op: Operation) {
        use Operation::*;
        match op {
            ClearScreen => self.display.clear(),
            Jump { address } => self.pc = address,
            JumpOffset {
                address,
                offset_register: _,
            } => {
                self.pc = address;
            }
            SetLiteral { dest, value } => self.variables[dest] = value,
            AddLiteral { dest, value } => {
                self.variables[dest] = self.variables[dest].wrapping_add(value)
            }
            SetIndex { src } => self.index = src,
            SetIndexFont { src } => {
                let character = (self.variables[src] & 0x0F) as usize;
                self.index = FONT_ADDRESS + character;
            }
            AddIndex { src } => {
                let (res, overflow) = self.index.overflowing_add(self.variables[src] as usize);
                self.index = res;
                self.variables[0xF] = overflow as u8;
            }
            Call { address } => {
                self.stack.push(self.pc);
                self.pc = address;
            }
            Return => {
                self.pc = self.stack.pop().expect("Don't pop out of main");
            }
            SkipEq { x, y } => {
                if self.variables[x] == self.variables[y] {
                    self.pc += 2;
                }
            }
            SkipNotEq { x, y } => {
                if self.variables[x] != self.variables[y] {
                    self.pc += 2;
                }
            }
            SkipEqLiteral { x, value } => {
                if self.variables[x] == value {
                    self.pc += 2;
                }
            }
            SkipNotEqLiteral { x, value } => {
                if self.variables[x] != value {
                    self.pc += 2;
                }
            }
            SkipIfKey { key_register } => {
                unimplemented!("SkipIfKey");
            }
            SkipIfNotKey { key_register } => {
                unimplemented!("SkipIfNotKey");
            }
            GetKey { dest } => {
                unimplemented!("GetKey");
            }
            Set { dest, src } => {
                self.variables[dest] = self.variables[src];
            }
            Or { lhs, rhs } => {
                self.variables[lhs] |= self.variables[rhs];
            }
            And { lhs, rhs } => {
                self.variables[lhs] &= self.variables[rhs];
            }
            Xor { lhs, rhs } => {
                self.variables[lhs] ^= self.variables[rhs];
            }
            Add { lhs, rhs } => {
                let (res, overflow) = self.variables[lhs].overflowing_add(self.variables[rhs]);
                self.variables[lhs] = res;
                self.variables[0xF] = overflow as u8;
            }
            Sub { lhs, rhs, dest } => {
                let (res, overflow) = self.variables[lhs].overflowing_sub(self.variables[rhs]);
                self.variables[dest] = res;
                self.variables[0xF] = !overflow as u8;
            }
            // TODO: Make ambiguity configurable
            LeftShift { lhs, rhs } => {
                self.variables[0xF] = self.variables[lhs] & 0b1000_0000;
                self.variables[lhs] <<= 1;
            }
            RightShift { lhs, rhs } => {
                self.variables[0xF] = self.variables[lhs] & 0b0000_0001;
                self.variables[lhs] >>= 1;
            }
            GetDelay { dest } => {
                self.variables[dest] = self.delay_timer;
            }
            SetDelay { src } => {
                self.delay_timer = self.variables[src];
            }
            SetSound { src } => {
                self.sound_timer = self.variables[src];
            }
            Draw {
                x,
                y,
                sprite_height,
            } => {
                let x = self.variables[x] as usize % WIDTH;
                let y = self.variables[y] as usize % HEIGHT;
                self.variables[0xF] = 0;

                for y_offset in 0..sprite_height {
                    if y + y_offset >= HEIGHT {
                        break;
                    }
                    let sprite_row = self.memory[self.index + y_offset];
                    for x_offset in 0..8 {
                        if x + x_offset >= WIDTH {
                            break;
                        }
                        let pixel = (sprite_row >> (7 - x_offset)) & 1;
                        let buffer_index = (y + y_offset) * WIDTH + (x + x_offset);
                        self.display.buffer[buffer_index] ^= pixel as u32 * 0xffffff;
                        self.variables[0xF] = pixel;
                    }
                }
            }
            Random { x, mask } => {
                self.variables[x] = random::<u8>() & mask;
            }
            DecimalConversion { src } => {
                unimplemented!("DecimalConversion");
            }
            StoreMemory { registers } => {
                unimplemented!("StoreMemory");
            }
            LoadMemory { registers } => {
                unimplemented!("LoadMemory")
            }
        }
    }
}
