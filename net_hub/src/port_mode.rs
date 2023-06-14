#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PortMode {
    Simple = 0,
    Complex = 1,
}

impl From<u8> for PortMode {
    fn from(value: u8) -> Self {
        if value >= Self::Simple as u8 && value <= Self::Complex as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Simple
        }
    }
}
