mod chip8;
mod instruction;
// mod schip;

use anyhow::Result;
pub use chip8::Chip8;
// pub use schip::Schip;

pub trait Interpreter {
    // fn display_open(&self) -> bool;
    fn tick(&mut self) -> Result<()>;
}
