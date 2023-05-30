#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum HintingLevel {
    NoHinting = 0,
    Slight = 1,
    Normal = 2,
    Full = 3,
}

impl From<u8> for HintingLevel {
    fn from(value: u8) -> Self {
        if value >= Self::NoHinting as u8 && value <= Self::Full as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::NoHinting
        }
    }
}
