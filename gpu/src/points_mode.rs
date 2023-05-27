#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum PointMode {
    Points = 0,
    Lines = 1,
    Polygons = 2,
}

impl From<i64> for PointMode {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Lines,
            2 => Self::Polygons,
            _ => Self::Points,
        }
    }
}
