use super::{Display, Input};
use minifb::{Key, KeyRepeat, Window};

impl Display for Window {
    fn is_open(&self) -> bool {
        self.is_open()
    }

    fn update_buffer(&mut self, buffer: &[u8], width: usize, height: usize) {
        let buffer = buffer
            .iter()
            .map(|&pixel| pixel as u32 * 0xffffff)
            .collect::<Vec<u32>>();
        self.update_with_buffer(&buffer, width, height)
            .unwrap_or_else(|err| eprintln!("Failed to update display buffer: {}", err));
    }

    fn update(&mut self) {
        self.update();
    }
}

fn keyval_to_key(key: u8) -> Option<Key> {
    use Key::*;
    match key {
        0x1 => Some(Key1),
        0x2 => Some(Key2),
        0x3 => Some(Key3),
        0xC => Some(Key4),

        0x4 => Some(Q),
        0x5 => Some(W),
        0x6 => Some(E),
        0xD => Some(R),

        0x7 => Some(A),
        0x8 => Some(S),
        0x9 => Some(D),
        0xE => Some(F),

        0xA => Some(Z),
        0x0 => Some(X),
        0xB => Some(C),
        0xF => Some(V),

        _ => None,
    }
}

fn key_to_keyval(key: &Key) -> Option<u8> {
    use Key::*;
    match key {
        Key1 => Some(0x1),
        Key2 => Some(0x2),
        Key3 => Some(0x3),
        Key4 => Some(0xC),

        Q => Some(0x4),
        W => Some(0x5),
        E => Some(0x6),
        R => Some(0xD),

        A => Some(0x7),
        S => Some(0x8),
        D => Some(0x9),
        F => Some(0xE),

        Z => Some(0xA),
        X => Some(0x0),
        C => Some(0xB),
        V => Some(0xF),

        _ => None,
    }
}

impl Input for Window {
    fn is_key_down(&self, keyval: u8) -> bool {
        self.is_key_down(keyval_to_key(keyval).expect("Should not be called with bad keyval"))
    }

    fn get_key(&self) -> Option<u8> {
        self.get_keys_released()
            .iter()
            .filter_map(key_to_keyval)
            .nth(0)
    }
}
