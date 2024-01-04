// Copyright Nicolas-Ștefan Bratoveanu, 2023,
// licensed under the EUPL-1.2-or-later

use anyhow::Result;
use clap::Parser;
use std::io::Read;
use std::{fs::File, path::Path};

use app::Octarou;
use args::Args;
use interpreter::Chip8;

use eframe::egui;

pub mod app;
pub mod args;
pub mod interpreter;
pub mod window;

fn main() -> Result<()> {
    let args = Args::parse();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 320.0]),
        ..Default::default()
    };

    let program = read_program_from_file(args.program.as_path())?;
    let app = Octarou::new(Chip8::new(args.cpu_speed, Some(&program)));

    eframe::run_native("Octarou", options, Box::new(move |_| Box::new(app))).unwrap();

    Ok(())
}

fn read_program_from_file(path: &Path) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
