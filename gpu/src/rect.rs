use crate::{MutPositionable, Point, Positionable};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Rect {
    #[inline]
    #[must_use]
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
    #[must_use]
    pub fn new_from_zero(width: f64, height: f64) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: width,
            bottom: height,
        }
    }

    #[inline]
    #[must_use]
    pub fn new_from_position(position: Point, width: f64, height: f64) -> Self {
        Self {
            left: position.x,
            top: position.y,
            right: position.x + width,
            bottom: position.y + height,
        }
    }
}

impl Positionable for Rect {
    #[inline]
    fn position(&self) -> Point {
        Point::new(self.left, self.top)
    }

    #[inline]
    fn x(&self) -> f64 {
        self.position().x
    }

    #[inline]
    fn y(&self) -> f64 {
        self.position().y
    }
}

impl MutPositionable for Rect {
    #[inline]
    fn set_position(&mut self, position: Point) {
        let width = self.width();
        let height = self.height();

        self.left = position.x;
        self.top = position.y;
        self.right = self.left + width;
        self.bottom = self.top + height;
    }

    #[inline]
    fn with_position(mut self, position: Point) -> Self {
        self.set_position(position);

        self
    }
}

pub trait Boundable: Positionable {
    #[must_use]
    fn bounds(&self) -> &Rect;

    #[must_use]
    fn left(&self) -> f64;

    #[must_use]
    fn right(&self) -> f64;

    #[must_use]
    fn top(&self) -> f64;

    #[must_use]
    fn bottom(&self) -> f64;

    #[must_use]
    fn width(&self) -> f64;

    #[must_use]
    fn height(&self) -> f64;
}

pub trait MutBoundable: Boundable + Positionable {
    fn set_bounds(&mut self, bounds: Rect);

    #[must_use]
    fn with_bounds(self, bounds: Rect) -> Self;
}

impl Boundable for Rect {
    #[inline]
    fn bounds(&self) -> &Rect {
        self
    }

    #[inline]
    fn left(&self) -> f64 {
        self.left
    }

    #[inline]
    fn right(&self) -> f64 {
        self.right
    }

    #[inline]
    fn top(&self) -> f64 {
        self.top
    }

    #[inline]
    fn bottom(&self) -> f64 {
        self.bottom
    }

    #[inline]
    fn width(&self) -> f64 {
        self.right - self.left
    }

    #[inline]
    fn height(&self) -> f64 {
        self.bottom - self.top
    }
}

impl MutBoundable for Rect {
    #[inline]
    fn set_bounds(&mut self, bounds: Rect) {
        assert!(bounds.left <= bounds.right);
        assert!(bounds.top <= bounds.bottom);

        *self = bounds;
    }

    #[inline]
    fn with_bounds(mut self, bounds: Rect) -> Self {
        self.set_bounds(bounds);

        self
    }
}

pub trait BoundableExt: Boundable {
    #[must_use]
    fn hcenter(&self) -> f64;

    #[must_use]
    fn vcenter(&self) -> f64;

    #[must_use]
    fn center(&self) -> Point;
}

impl<T> BoundableExt for T
where
    T: Boundable,
{
    #[inline]
    fn hcenter(&self) -> f64 {
        self.width() / 2.0 + self.left()
    }

    #[inline]
    fn vcenter(&self) -> f64 {
        self.height() / 2.0 + self.left()
    }

    #[inline]
    fn center(&self) -> Point {
        let x = self.hcenter();
        let y = self.vcenter();

        Point::new(x, y)
    }
}
