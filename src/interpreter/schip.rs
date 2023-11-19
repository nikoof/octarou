use super::{instruction::Instruction, Interpreter};
use crate::window::{Display, Input};
use anyhow::{anyhow, Result};
use std::time::{Duration, Instant};

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

const DEFAULT_SPEED: u64 = 700;

pub struct Schip<W>
where
    W: Display + Input,
{
    memory: [u8; 4096],
    pc: usize,
    index: usize,
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
    variables: [u8; 16],
    display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    speed: u64,
    window: W,
}

impl<W> Default for Schip<W>
where
    W: Display + Input + Default,
{
    fn default() -> Self {
        let mut window = W::default();
        window.set_update_rate(DEFAULT_SPEED);
        Self::new(window, DEFAULT_SPEED, None)
    }
}

impl<W> Interpreter for Schip<W>
where
    W: Display + Input,
{
    fn display_open(&self) -> bool {
        self.window.is_open()
    }

    fn tick(&mut self) -> Result<()> {
        let timer_cycle_duration = Duration::from_nanos(1_000_000_000 / 60);
        let cpu_cycle_duration = Duration::from_nanos(1_000_000_000 / self.speed);

        let now = Instant::now();
        let mut total_elapsed = Duration::from_secs(0);

        self.update_timers();

        'cpu: loop {
            let next_instruction = self.next_instruction()?;
            self.execute_instruction(next_instruction);
            self.window
                .update_buffer(&self.display, DISPLAY_WIDTH, DISPLAY_HEIGHT);

            let cpu_elapsed = now.elapsed() - total_elapsed;
            total_elapsed += cpu_elapsed;

            if cpu_elapsed < cpu_cycle_duration {
                let time_left = cpu_cycle_duration - cpu_elapsed;
                total_elapsed += time_left;
                std::thread::sleep(time_left);
            }

            if total_elapsed >= timer_cycle_duration {
                break 'cpu;
            }
        }

        Ok(())
    }
}

impl<W> Schip<W>
where
    W: Display + Input,
{
    pub fn new(mut display: W, speed: u64, program: Option<&[u8]>) -> Self {
        display.set_update_rate(speed);

        let mut memory = [0u8; 4096];
        memory[FONT_ADDRESS..FONT_ADDRESS + FONT.len()].copy_from_slice(&FONT);
        if let Some(program) = program {
            memory[PROGRAM_ADDRESS..PROGRAM_ADDRESS + program.len()].copy_from_slice(program);
        }

        Self {
            memory,
            pc: PROGRAM_ADDRESS,
            index: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            variables: [0; 16],
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            speed,
            window: display,
        }
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

    fn execute_instruction(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            ClearScreen => self.display.fill(0),
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
                if self.window.is_key_down(self.variables[key_register]) {
                    self.pc += 2;
                }
            }
            SkipIfNotKey { key_register } => {
                if !self.window.is_key_down(self.variables[key_register]) {
                    self.pc += 2;
                }
            }
            GetKey { dest } => {
                if let Some(key) = self.window.get_key() {
                    self.variables[dest] = key as u8;
                } else {
                    self.pc -= 2;
                }
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
                let flag = self.variables[lhs] >> 7;
                self.variables[lhs] <<= 1;
                self.variables[0xF] = flag;
            }
            RightShift { lhs, rhs } => {
                let flag = self.variables[lhs] & 1;
                self.variables[lhs] >>= 1;
                self.variables[0xF] = flag;
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
                        let buffer_index = (y + y_offset) * DISPLAY_WIDTH + (x + x_offset);
                        self.variables[0xF] |= self.display[buffer_index] & pixel;
                        self.display[buffer_index] ^= pixel;
                    }
                }
            }
            Random { x, mask } => {
                self.variables[x] = rand::random::<u8>() & mask;
            }
            DecimalConversion { src } => {
                let mut n = self.variables[src];

                for i in (0..3).rev() {
                    self.memory[self.index + i] = n % 10;
                    n /= 10;
                }
            }
            StoreMemory { registers } => {
                for i in 0..=registers {
                    self.memory[self.index + i] = self.variables[i];
                }
            }
            LoadMemory { registers } => {
                for i in 0..=registers {
                    self.variables[i] = self.memory[self.index + i];
                }
            }
        }
    }
}
