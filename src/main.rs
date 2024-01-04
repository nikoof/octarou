// Copyright Nicolas-Ștefan Bratoveanu, 2023,
// licensed under the EUPL-1.2-or-later

use anyhow::Result;
use clap::Parser;
use std::io::Read;
use std::{fs::File, path::Path};

use args::{Args, Variant};
use chip::{Chip8, Interpreter, Schip};

pub mod args;
pub mod chip;

fn main() -> Result<()> {
    let args = Args::parse();

    let program = read_program_from_file(args.program.as_path())?;
    let mut state: Box<dyn Interpreter> = match args.variant {
        Variant::Chip8 => Box::new(Chip8::new(args.cpu_speed, Some(&program))),
        Variant::Schip => Box::new(Schip::new(args.cpu_speed, Some(&program))),
    };

    Ok(())
}

fn read_program_from_file(path: &Path) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
