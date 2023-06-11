use core::ops::{Add, AddAssign, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Add for Point {
    type Output = Point;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub trait Positionable {
    #[must_use]
    fn position(&self) -> Point;

    #[inline]
    #[must_use]
    fn x(&self) -> f64 {
        self.position().x
    }

    #[inline]
    #[must_use]
    fn y(&self) -> f64 {
        self.position().y
    }
}

pub trait MutPositionable: Positionable {
    fn set_position(&mut self, position: Point);

    #[must_use]
    fn with_position(self, position: Point) -> Self;

    #[inline]
    fn translate_x(&mut self, x: f64) {
        let mut position = self.position();
        position.x += x;

        self.set_position(position);
    }

    #[inline]
    fn translate_y(&mut self, y: f64) {
        let mut position = self.position();
        position.y += y;

        self.set_position(position);
    }

    fn translate(&mut self, point: Point) {
        self.set_position(self.position() + point);
    }
}
