use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buf = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buf.iter_mut().enumerate() {
            let x: u32 = (i % WIDTH) as u32;
            let y: u32 = (i / WIDTH) as u32;
            *pixel = (x % (y + 1)) % u32::MAX;
        }

        window.update_with_buffer(&buf, WIDTH, HEIGHT).unwrap()
    }
}
