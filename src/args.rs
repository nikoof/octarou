use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Variant {
    Chip8,
    Schip,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::Chip8 => "chip8",
            Self::Schip => "schip",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to a file containing a CHIP-8 program.
    pub program: std::path::PathBuf,

    /// CHIP-8 variant to interpret
    #[arg(short = 'v', long, default_value_t = Variant::Chip8)]
    pub variant: Variant,

    /// Window width
    #[arg(short = 'W', long, default_value_t = 640)]
    pub window_width: usize,

    /// Window height
    #[arg(short = 'H', long, default_value_t = 320)]
    pub window_height: usize,

    /// Speed of CPU (in CH8-ops/second).
    #[arg(short = 's', long, default_value_t = 700)]
    pub cpu_speed: u64,
}
