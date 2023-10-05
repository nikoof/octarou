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

impl Input for Window {
    fn get_keys(&self, keys: &mut [bool]) {
        for key in 0u8..=0xF {
            keys[key as usize] = self.is_key_pressed(
                keyval_to_key(key).expect("All cases should be covered"),
                KeyRepeat::No,
            )
        }
    }
}
