pub struct Display {
    buffer: [u32; 64 * 32],
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [0; 64 * 32],
        }
    }
}
