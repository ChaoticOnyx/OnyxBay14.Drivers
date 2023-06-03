#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn red() -> Self {
        Self::new_rgb(255, 0, 0)
    }

    pub const fn green() -> Self {
        Self::new_rgb(0, 255, 0)
    }

    pub const fn blue() -> Self {
        Self::new_rgb(0, 0, 255)
    }

    pub const fn white() -> Self {
        Self::new_rgb(255, 255, 255)
    }

    pub const fn black() -> Self {
        Self::new_rgb(0, 0, 0)
    }
}

impl From<u64> for Color {
    fn from(color: u64) -> Self {
        Color::new_rgba(
            (color >> 24) as u8,
            (color >> 16) as u8,
            (color >> 8) as u8,
            color as u8,
        )
    }
}

impl From<f64> for Color {
    fn from(color: f64) -> Self {
        Self::from(color as u64)
    }
}
