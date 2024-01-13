use super::{instruction::Instruction, Interpreter};
use anyhow::{anyhow, Result};

const PROGRAM_ADDRESS: usize = 0x200;
const FONT_ADDRESS: usize = 0x50;
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

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Chip8 {
    memory: [u8; 4096],
    pc: usize,
    index: usize,
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
    variables: [u8; 16],
    display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Chip8 {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0u8; 4096];
        memory[FONT_ADDRESS..FONT_ADDRESS + FONT.len()].copy_from_slice(&FONT);
        memory[PROGRAM_ADDRESS..PROGRAM_ADDRESS + program.len()].copy_from_slice(program);

        Self {
            memory,
            pc: PROGRAM_ADDRESS,
            index: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            variables: [0; 16],
            display: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }
}

impl Interpreter for Chip8 {
    fn display(&self) -> Vec<&[u8]> {
        self.display.iter().map(|row| row.as_slice()).collect()
    }

    fn update_timers(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    fn next_instruction(&mut self) -> Result<Instruction> {
        let opcode = self.memory[self.pc..self.pc + 2].try_into()?;
        let opcode = u16::from_be_bytes(opcode);
        self.pc += 2;
        Instruction::new(opcode).ok_or(anyhow!("Cannot decode opcode {:#06x}", opcode))
    }

    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        keys_down: &[bool; 16],
        keys_released: &[bool; 16],
    ) -> Result<()> {
        use Instruction::*;
        match instruction {
            ClearScreen => Ok(self.display.iter_mut().for_each(|e| e.fill(0))),
            Jump { address } => Ok(self.pc = address),
            JumpOffset {
                address,
                offset_register: _,
            } => {
                self.pc = address + self.variables[0] as usize;
                Ok(())
            }
            SetLiteral { dest, value } => Ok(self.variables[dest] = value),
            AddLiteral { dest, value } => {
                self.variables[dest] = self.variables[dest].wrapping_add(value);
                Ok(())
            }
            SetIndex { src } => Ok(self.index = src),
            SetIndexFont { src, big: _ } => {
                let character = (self.variables[src] & 0x0F) as usize;
                self.index = FONT_ADDRESS + 5 * character;
                Ok(())
            }
            AddIndex { src } => {
                let (res, overflow) = self.index.overflowing_add(self.variables[src] as usize);
                self.index = res;
                self.variables[0xF] = overflow as u8;
                Ok(())
            }
            Call { address } => {
                self.stack.push(self.pc);
                self.pc = address;
                Ok(())
            }
            Return => {
                self.pc = self.stack.pop().expect("Don't pop out of main");
                Ok(())
            }
            SkipEq { x, y } => {
                if self.variables[x] == self.variables[y] {
                    self.pc += 2;
                }
                Ok(())
            }
            SkipNotEq { x, y } => {
                if self.variables[x] != self.variables[y] {
                    self.pc += 2;
                }
                Ok(())
            }
            SkipEqLiteral { x, value } => {
                if self.variables[x] == value {
                    self.pc += 2;
                }
                Ok(())
            }
            SkipNotEqLiteral { x, value } => {
                if self.variables[x] != value {
                    self.pc += 2;
                }
                Ok(())
            }
            SkipIfKey { key_register } => {
                if keys_down[self.variables[key_register] as usize] {
                    self.pc += 2;
                }
                Ok(())
            }
            SkipIfNotKey { key_register } => {
                if !keys_down[self.variables[key_register] as usize] {
                    self.pc += 2;
                }
                Ok(())
            }
            GetKey { dest } => {
                if let Some(key) = keys_released.iter().position(|&e| e) {
                    self.variables[dest] = key as u8;
                } else {
                    self.pc -= 2;
                }
                Ok(())
            }
            Set { dest, src } => {
                self.variables[dest] = self.variables[src];
                Ok(())
            }
            Or { lhs, rhs } => {
                self.variables[lhs] |= self.variables[rhs];
                Ok(())
            }
            And { lhs, rhs } => {
                self.variables[lhs] &= self.variables[rhs];
                Ok(())
            }
            Xor { lhs, rhs } => {
                self.variables[lhs] ^= self.variables[rhs];
                Ok(())
            }
            Add { lhs, rhs } => {
                let (res, overflow) = self.variables[lhs].overflowing_add(self.variables[rhs]);
                self.variables[lhs] = res;
                self.variables[0xF] = overflow as u8;
                Ok(())
            }
            Sub { lhs, rhs, dest } => {
                let (res, overflow) = self.variables[lhs].overflowing_sub(self.variables[rhs]);
                self.variables[dest] = res;
                self.variables[0xF] = !overflow as u8;
                Ok(())
            }
            LeftShift { lhs, rhs } => {
                self.variables[lhs] = self.variables[rhs];
                let flag = self.variables[lhs] >> 7;
                self.variables[lhs] <<= 1;
                self.variables[0xF] = flag;
                Ok(())
            }
            RightShift { lhs, rhs } => {
                self.variables[lhs] = self.variables[rhs];
                let flag = self.variables[lhs] & 1;
                self.variables[lhs] >>= 1;
                self.variables[0xF] = flag;
                Ok(())
            }
            GetDelay { dest } => {
                self.variables[dest] = self.delay_timer;
                Ok(())
            }
            SetDelay { src } => {
                self.delay_timer = self.variables[src];
                Ok(())
            }
            SetSound { src } => {
                self.sound_timer = self.variables[src];
                Ok(())
            }
            Draw {
                x,
                y,
                sprite_height,
            } => {
                let x = self.variables[x] as usize % DISPLAY_WIDTH;
                let y = self.variables[y] as usize % DISPLAY_HEIGHT;
                self.variables[0xF] = 0;

                for y_offset in 0..sprite_height {
                    if y + y_offset >= DISPLAY_HEIGHT {
                        break;
                    }
                    let sprite_row = self.memory[self.index + y_offset];
                    for x_offset in 0..8 {
                        if x + x_offset >= DISPLAY_WIDTH {
                            break;
                        }
                        let pixel = (sprite_row >> (7 - x_offset)) & 1;
                        self.variables[0xF] |= self.display[y + y_offset][x + x_offset] & pixel;
                        self.display[y + y_offset][x + x_offset] ^= pixel;
                    }
                }
                Ok(())
            }
            Random { x, mask } => {
                self.variables[x] = rand::random::<u8>() & mask;
                Ok(())
            }
            DecimalConversion { src } => {
                let mut n = self.variables[src];

                for i in (0..3).rev() {
                    self.memory[self.index + i] = n % 10;
                    n /= 10;
                }
                Ok(())
            }
            StoreMemory { registers } => {
                for i in 0..=registers {
                    self.memory[self.index + i] = self.variables[i];
                    // self.index = i;
                }
                Ok(())
            }
            LoadMemory { registers } => {
                for i in 0..=registers {
                    self.variables[i] = self.memory[self.index + i];
                    // self.index = i;
                }
                Ok(())
            }
            _ => Err(anyhow!(
                "Instruction {:?} not in Chip8 instruction set.",
                instruction
            )),
        }
    }
}
