#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyState {
    Up = 0,
    Repeat = 1,
    Down = 2,
}

impl From<u8> for KeyState {
    fn from(value: u8) -> Self {
        if value >= KeyState::Up as u8 && value <= KeyState::Down as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            KeyState::Up
        }
    }
}
