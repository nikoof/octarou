mod minifb;

pub trait Display {
    fn is_open(&self) -> bool;
    fn update_buffer(&mut self, new_buffer: &[u8], width: usize, height: usize);
    fn update(&mut self);
}

pub trait Input {
    fn get_keys(&self, keys: &mut [bool]);
}
