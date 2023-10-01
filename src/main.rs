use std::io::Read;
use std::{fs::File, path::Path};

use state::State;

pub mod display;
pub mod operation;
pub mod state;

fn main() {
    let mut state = State::default();
    if let Ok(program) = read_program_from_file(Path::new("./roms/programs/IBM Logo.ch8")) {
        state.load_program(&program);
    }

    loop {
        let op = state.next_operation();
        state.update(op);
    }
}

fn read_program_from_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
