mod chip8;
mod instruction;
mod superchip;

use instruction::Instruction;
use std::time;
use thiserror::Error;

pub use chip8::Chip8;
pub use superchip::Superchip;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Unknown opcode {opcode:#06x} at {address:#06x}")]
    UnknownOpcode { opcode: u16, address: usize },

    #[error("Attempted to pop out of an empty callstack")]
    PopOutOfMain,

    #[error("Program counter was incremented beyond maximum memory size")]
    OutOfMemory,

    #[error("Instruction {instruction:?} not in CHIP-8 instruction set")]
    Chip8InvalidInstruction { instruction: Instruction },

    #[allow(unused)]
    #[error("Instruction {instruction:?} not in SUPERCHIP instruction set")]
    SuperchipInvalidInstruction { instruction: Instruction },
}

pub trait Interpreter {
    fn display(&self) -> Vec<&[u8]>;

    fn update_timers(&mut self);
    fn next_instruction(&mut self) -> Result<Instruction, InterpreterError>;
    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        keys_pressed: &[bool; 16],
        keys_released: &[bool; 16],
    ) -> Result<(), InterpreterError>;

    #[cfg(not(target_arch = "wasm32"))]
    fn tick(
        &mut self,
        keys_down: &[bool; 16],
        keys_released: &[bool; 16],
        speed: u64,
    ) -> Result<(), InterpreterError> {
        let timer_cycle_duration = time::Duration::from_nanos(1_000_000_000 / 60);
        let cpu_cycle_duration = time::Duration::from_nanos(1_000_000_000 / speed);

        let now = time::Instant::now();
        let mut total_elapsed = time::Duration::from_secs(0);

        self.update_timers();

        'cpu: loop {
            let next_instruction = self.next_instruction()?;
            self.execute_instruction(next_instruction, keys_down, keys_released)?;

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

    // There is no timing in wasm32 since there is no support for std::time::Instant on this
    // platform. I have tried using wasm_bindgen bindings for JS's Performance.now(), but that
    // makes the app lag heavily for some reason. For now at least it can load roms.
    #[cfg(target_arch = "wasm32")]
    fn tick(
        &mut self,
        keys_down: &[bool; 16],
        keys_released: &[bool; 16],
        speed: u64,
    ) -> Result<(), InterpreterError> {
        let timer_cycle_duration = time::Duration::from_nanos(1_000_000_000 / 60);
        let cpu_cycle_duration = time::Duration::from_nanos(1_000_000_000 / speed);

        let then = time::Duration::from_millis((eframe::web::now_sec() * 1000.0) as u64);
        let mut total_elapsed = time::Duration::from_secs(0);

        self.update_timers();

        'cpu: loop {
            let next_instruction = self.next_instruction()?;
            self.execute_instruction(next_instruction, keys_down, keys_released)?;

            let now = time::Duration::from_millis((eframe::web::now_sec() * 1000.0) as u64);
            let cpu_elapsed = now - then - total_elapsed;
            total_elapsed += cpu_elapsed;

            if total_elapsed >= timer_cycle_duration {
                break 'cpu;
            }
        }

        Ok(())
    }
}
