use gpu::{Point, Rect};

use crate::Sgl;

#[derive(Debug, Clone)]
pub struct Image {
    bounds: Rect,
    object_id: u64,
}

impl Image {
    pub fn new(data: &'static [u8], bounds: Rect, sgl: &mut Sgl) -> Self {
        let object_id = sgl.create_image(data, bounds.width() as u64, bounds.height() as u64);

        Self { bounds, object_id }
    }

    pub fn new_from_raw(bounds: Rect, object_id: u64) -> Self {
        Self { bounds, object_id }
    }

    pub fn id(&self) -> u64 {
        self.object_id
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.set_position(position);

        self
    }

    pub fn set_position(&mut self, position: Point) {
        self.bounds.set_position(position)
    }

    pub fn with_bounds(mut self, bounds: Rect) -> Self {
        self.set_bounds(bounds);

        self
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    pub fn draw(&self, sgl: &mut Sgl) {
        sgl.draw_image(self.object_id, self.bounds.position());
    }

    pub fn draw_rect(&self, sgl: &mut Sgl) {
        sgl.draw_image_rect(self.object_id, self.bounds);
    }

    pub fn position(&self) -> Point {
        self.bounds.position()
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    pub fn translate_x(&mut self, x: f64) {
        self.bounds.translate_x(x);
    }

    pub fn translate_y(&mut self, y: f64) {
        self.bounds.translate_y(y);
    }

    pub fn destroy(self, sgl: &mut Sgl) {
        sgl.delete_object(self.object_id)
    }
}
