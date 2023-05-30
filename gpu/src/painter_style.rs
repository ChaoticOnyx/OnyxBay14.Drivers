#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PainterStyle {
    Fill = 0,
    Stroke = 1,
    StrokeAndFill = 2,
}

impl From<u8> for PainterStyle {
    fn from(value: u8) -> Self {
        if value >= Self::Fill as u8 && value <= Self::StrokeAndFill as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Fill
        }
    }
}
