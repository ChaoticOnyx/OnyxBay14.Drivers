pub const KEYS_OFFSET: usize = 0x100;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseKey {
    Left = 0,
    Right = 1,
    Middle = 2,
    Unknown = 0xFF,
}

impl MouseKey {
    pub(crate) fn offset(&self) -> usize {
        *self as usize + KEYS_OFFSET
    }
}

impl From<u8> for MouseKey {
    fn from(value: u8) -> Self {
        if value >= MouseKey::Left as u8 && value <= MouseKey::Unknown as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            MouseKey::Unknown
        }
    }
}
