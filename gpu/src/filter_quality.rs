#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FilterQuality {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

impl From<u8> for FilterQuality {
    fn from(value: u8) -> Self {
        if value >= Self::None as u8 && value <= Self::High as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::None
        }
    }
}
