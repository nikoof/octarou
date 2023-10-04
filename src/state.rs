use crate::display::{Display, HEIGHT, WIDTH};
use crate::operation::Operation;
use anyhow::Result;

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
    stack: Vec<u16>,
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
        self.memory[0x200..0x200 + program.len()].copy_from_slice(&program);
        self.pc = 0x200;
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
            SetLiteral { destination, value } => self.variables[destination] = value,
            AddLiteral { destination, value } => self.variables[destination] += value,
            SetIndex { source } => self.index = source,
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
            _ => (),
        }
    }
}
