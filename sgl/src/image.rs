use gpu::{Boundable, MutBoundable, MutPositionable, Point, Positionable, Rect};

#[derive(Debug, Clone)]
pub struct Image {
    bounds: Rect,
    object_id: u64,
}

impl Image {
    #[inline]
    pub(crate) fn new_from_id(bounds: Rect, object_id: u64) -> Self {
        Self { bounds, object_id }
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.object_id
    }
}

impl Positionable for Image {
    #[inline]
    fn position(&self) -> Point {
        self.bounds.position()
    }
}

impl MutPositionable for Image {
    #[inline]
    fn set_position(&mut self, position: Point) {
        self.bounds.set_position(position);
    }

    #[inline]
    fn with_position(mut self, position: Point) -> Self {
        self.bounds.set_position(position);

        self
    }
}

impl Boundable for Image {
    #[inline]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    #[inline]
    fn left(&self) -> f64 {
        self.bounds.left
    }

    #[inline]
    fn right(&self) -> f64 {
        self.bounds.right
    }

    #[inline]
    fn top(&self) -> f64 {
        self.bounds.top
    }

    #[inline]
    fn bottom(&self) -> f64 {
        self.bounds.bottom
    }

    #[inline]
    fn width(&self) -> f64 {
        self.bounds.width()
    }

    #[inline]
    fn height(&self) -> f64 {
        self.bounds.height()
    }
}

impl MutBoundable for Image {
    #[inline]
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds.set_bounds(bounds);
    }

    #[inline]
    fn with_bounds(mut self, bounds: Rect) -> Self {
        self.bounds = self.bounds.with_bounds(bounds);

        self
    }
}
