use anyhow::Result;
use std::io::Read;
use std::{fs::File, path::Path};

use state::State;

pub mod display;
pub mod operation;
pub mod state;

fn main() -> Result<()> {
    let mut state = State::default()?;

    let program = read_program_from_file(Path::new("./roms/programs/IBM Logo.ch8"))?;
    state.load_program(&program);

    while state.should_run() {
        state.tick();
    }

    Ok(())
}

fn read_program_from_file(path: &Path) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
