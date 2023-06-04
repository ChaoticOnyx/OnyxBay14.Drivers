use crate::Point;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Rect {
    #[inline]
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        assert!(left <= right);
        assert!(top <= bottom);

        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn new_from_zero(width: f64, height: f64) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: width,
            bottom: height,
        }
    }

    #[inline]
    pub fn new_from_position(position: Point, width: f64, height: f64) -> Self {
        Self {
            left: position.x,
            top: position.y,
            right: position.x + width,
            bottom: position.y + height,
        }
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    #[inline]
    pub fn hcenter(&self) -> f64 {
        self.width() / 2.0 + self.left
    }

    #[inline]
    pub fn vcenter(&self) -> f64 {
        self.height() / 2.0 + self.left
    }

    #[inline]
    pub fn center(&self) -> Point {
        let x = self.hcenter();
        let y = self.vcenter();

        Point::new(x, y)
    }

    #[inline]
    pub fn translate_x(&mut self, x: f64) {
        self.left += x;
        self.right += x;
    }

    #[inline]
    pub fn translate_y(&mut self, y: f64) {
        self.top += y;
        self.bottom += y;
    }

    #[inline]
    pub fn position(&self) -> Point {
        Point::new(self.left, self.top)
    }

    pub fn set_position(&mut self, position: Point) {
        let width = self.width();
        let height = self.height();

        self.left = position.x;
        self.top = position.y;
        self.right = position.x + width;
        self.bottom = position.y + height;
    }
}
