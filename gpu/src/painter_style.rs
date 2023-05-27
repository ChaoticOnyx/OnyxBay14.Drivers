#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum PainterStyle {
    Fill = 0,
    Stroke = 1,
    StrokeAndFill = 2,
}

impl From<i64> for PainterStyle {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Stroke,
            2 => Self::StrokeAndFill,
            _ => Self::Fill,
        }
    }
}
