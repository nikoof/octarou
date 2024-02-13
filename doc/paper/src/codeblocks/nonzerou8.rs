#[derive(Debug)]
struct NonZeroU8(u8);

impl NonZeroU8 {
    fn new(value: u8) -> Option<Self> {
        match value {
            0 => None,
            _ => Some(Self(value))
        }
    }
}
