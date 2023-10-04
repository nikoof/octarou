use anyhow::Result;
use minifb::{Scale, Window, WindowOptions};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pub buffer: [u32; WIDTH * HEIGHT],
    pub window: Window,
}

impl Display {
    pub fn new() -> Result<Self> {
        let window = Window::new(
            "chip8",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: true,
                scale: Scale::X8,
                ..WindowOptions::default()
            },
        )?;

        Ok(Self {
            buffer: [0; WIDTH * HEIGHT],
            window,
        })
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
