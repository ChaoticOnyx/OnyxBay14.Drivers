#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PointMode {
    Points = 0,
    Lines = 1,
    Polygons = 2,
}

impl From<u8> for PointMode {
    fn from(value: u8) -> Self {
        if value >= Self::Points as u8 && value <= Self::Polygons as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Points
        }
    }
}
