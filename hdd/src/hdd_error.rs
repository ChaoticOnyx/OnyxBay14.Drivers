#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HddError {
    InvalidAddress = -1,
    InvalidSize = -2,
    Unknown = 0xFFFFFF,
}

impl From<i64> for HddError {
    fn from(value: i64) -> Self {
        if value >= Self::InvalidAddress as i64 && value <= Self::InvalidSize as i64 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Unknown
        }
    }
}
