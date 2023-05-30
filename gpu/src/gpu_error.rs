#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpuError {
    NotInitialized = -1,
    InvalidSize = -2,
    UnknownBuffer = -3,
    InvalidBuffer = -4,
    NotEnoughMemory = -5,
    Unknown = 0xFFFFFF,
}

impl From<i64> for GpuError {
    fn from(value: i64) -> Self {
        if value >= Self::NotInitialized as i64 && value <= Self::NotEnoughMemory as i64 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Unknown
        }
    }
}
