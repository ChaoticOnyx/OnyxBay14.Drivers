#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum HintingLevel {
    NoHinting = 0,
    Slight = 1,
    Normal = 2,
    Full = 3,
}

impl From<i64> for HintingLevel {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Slight,
            2 => Self::Normal,
            3 => Self::Full,
            _ => Self::NoHinting,
        }
    }
}
