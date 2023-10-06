use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to a file containing a CHIP-8 program.
    pub program: std::path::PathBuf,

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
