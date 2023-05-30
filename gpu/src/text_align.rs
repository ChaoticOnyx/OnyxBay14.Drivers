#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextAlign {
    Left = 0,
    Center,
    Right,
}

impl From<u8> for TextAlign {
    fn from(value: u8) -> Self {
        if value >= Self::Left as u8 && value <= Self::Right as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Left
        }
    }
}
