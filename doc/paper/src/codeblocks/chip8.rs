pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    pc: usize,
    index: usize,
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
    variables: [u8; 16],
    display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}
