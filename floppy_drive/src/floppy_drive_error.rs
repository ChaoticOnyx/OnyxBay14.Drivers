#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FloppyDriveError {
    InvalidAddress = -1,
    InvalidSize = -2,
    FloppyDriveIsEmpty = -3,
    Unknown = 0xFFFFFF,
}

impl From<i64> for FloppyDriveError {
    fn from(value: i64) -> Self {
        if value >= Self::InvalidAddress as i64 && value <= Self::FloppyDriveIsEmpty as i64 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Unknown
        }
    }
}
