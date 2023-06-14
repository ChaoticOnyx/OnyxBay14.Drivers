#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NetHubError {
    InvalidPort = -1,
    InvalidPortMode = -2,
    InvalidSize = -3,
    Unknown = 0xFFFFFF,
}

impl From<i64> for NetHubError {
    fn from(value: i64) -> Self {
        if value >= Self::InvalidPort as i64 && value <= Self::InvalidSize as i64 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Unknown
        }
    }
}
