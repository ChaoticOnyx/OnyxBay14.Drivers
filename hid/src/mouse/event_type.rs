#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseEventType {
    None = 0,
    Key = 1,
    Move = 2,
}

impl From<u8> for MouseEventType {
    fn from(value: u8) -> Self {
        if value >= MouseEventType::None as u8 && value <= MouseEventType::Move as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            MouseEventType::None
        }
    }
}
