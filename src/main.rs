// Copyright Nicolas-Ștefan Bratoveanu, 2023,
// licensed under the EUPL-1.2-or-later

use anyhow::Result;
use clap::Parser;
use minifb::{ScaleMode, Window, WindowOptions};
use std::io::Read;
use std::{fs::File, path::Path};

use args::{Args, Variant};
use chip::{Chip8, Interpreter, Schip};

pub mod args;
pub mod chip;
pub mod window;

fn main() -> Result<()> {
    let args = Args::parse();

    let window = Window::new(
        match args.program.file_name() {
            Some(s) => s.to_str().unwrap_or("CHIP-8"),
            None => "CHIP-8",
        },
        args.window_width,
        args.window_height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|err| panic!("Failed to create window: {}", err));

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
