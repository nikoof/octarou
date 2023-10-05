mod minifb;

pub trait Display {
    fn is_open(&self) -> bool;
    fn update_buffer(&mut self, new_buffer: &[u8], width: usize, height: usize);
    fn update(&mut self);
}

pub trait Input {
    fn is_key_down(&self, key: u8) -> bool;
    fn get_key(&self) -> Option<u8>;
}
