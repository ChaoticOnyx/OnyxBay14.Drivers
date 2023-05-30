#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ObjectType {
    Points = 0,
    Vertices = 1,
    Text = 2,
    Typeface = 3,
}

impl From<u8> for ObjectType {
    fn from(value: u8) -> Self {
        if value >= Self::Points as u8 && value <= Self::Typeface as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Points
        }
    }
}
