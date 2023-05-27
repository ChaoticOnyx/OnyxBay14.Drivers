#[repr(C)]
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
        match value {
            -1 => Self::NotInitialized,
            -2 => Self::InvalidSize,
            -3 => Self::UnknownBuffer,
            -4 => Self::InvalidBuffer,
            -5 => Self::NotEnoughMemory,
            _ => Self::Unknown,
        }
    }
}
