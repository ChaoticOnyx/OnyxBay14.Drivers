use gpu::{Color, Point, Rect, TextAlign};

use crate::{Sgl, Typeface};

#[derive(Debug, Clone)]
pub struct Text<T>
where
    T: AsRef<str>,
{
    text: TextType<T>,
    color: Option<Color>,
    size: Option<f64>,
    align: TextAlign,
    position: Point,
    typeface: Option<Typeface>,
}

#[derive(Debug, Clone)]
enum TextType<T>
where
    T: AsRef<str>,
{
    Static(u64),
    Dynamic(T),
}

impl<T> Text<T>
where
    T: AsRef<str>,
{
    pub const fn new_dynamic(text: T) -> Self {
        Self {
            text: TextType::Dynamic(text),
            color: None,
            size: None,
            align: TextAlign::Left,
            position: Point::zero(),
            typeface: None,
        }
    }

    pub fn new_static(text: T, sgl: &mut Sgl) -> Self {
        Self {
            text: TextType::Static(sgl.create_text_object(text.as_ref())),
            color: None,
            size: None,
            align: TextAlign::Left,
            position: Point::zero(),
            typeface: None,
        }
    }

    pub fn mesaure_width(&self, sgl: &mut Sgl) -> f64 {
        let mut old_size: Option<f64> = None;

        if let Some(size) = self.size {
            old_size = Some(sgl.swap_text_size(size));
        }

        let width = match self.text {
            TextType::Static(object_id) => sgl.mesaure_text(object_id),
            TextType::Dynamic(ref text) => sgl.measure_string(text.as_ref()),
        };

        if let Some(old_size) = old_size {
            sgl.set_painter_text_size(old_size);
        }

        width
    }

    pub fn measure_height(&self, sgl: &mut Sgl) -> f64 {
        let mut old_size: Option<f64> = None;
        let current_size;

        if let Some(size) = self.size {
            current_size = sgl.swap_text_size(size);
            old_size = Some(current_size);
        } else {
            current_size = sgl.get_painter_text_size();
        }

        if let Some(old_size) = old_size {
            sgl.set_painter_text_size(old_size);
        }

        current_size
    }

    pub fn calc_bounds(&self, sgl: &mut Sgl) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.position.x + self.mesaure_width(sgl),
            self.position.y + self.measure_height(sgl),
        )
    }

    pub fn draw(&self, sgl: &mut Sgl) {
        let mut position = self.position;
        let mut old_color: Option<Color> = None;
        let mut old_size: Option<f64> = None;
        let mut old_typeface: Option<u64> = None;
        let old_align = sgl.swap_painter_text_align(self.align);

        if let Some(color) = self.color {
            old_color = Some(sgl.swap_painter_color(color));
        }

        if let Some(size) = self.size {
            old_size = Some(sgl.swap_text_size(size));
        }

        if let Some(typeface) = self.typeface {
            old_typeface = Some(sgl.swap_painter_typeface(Some(typeface.id())));
        }

        let text_size = old_size.unwrap_or_else(|| sgl.get_painter_text_size());
        position.y += text_size;

        match self.text {
            TextType::Static(object_id) => sgl.draw_text(object_id, position),
            TextType::Dynamic(ref text) => sgl.draw_string(text.as_ref(), position),
        };

        if let Some(old_color) = old_color {
            sgl.set_painter_color(old_color);
        }

        if let Some(old_size) = old_size {
            sgl.set_painter_text_size(old_size);
        }

        if let Some(old_typeface) = old_typeface {
            sgl.set_painter_typeface(Some(old_typeface))
        }

        sgl.set_painter_text_align(old_align);
    }

    pub fn with_color(mut self, color: Option<Color>) -> Self {
        self.set_color(color);

        self
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
    }

    pub fn with_size(mut self, size: Option<f64>) -> Self {
        self.set_size(size);

        self
    }

    pub fn set_size(&mut self, size: Option<f64>) {
        self.size = size;
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.set_align(align);

        self
    }

    pub fn set_align(&mut self, align: TextAlign) {
        self.align = align;
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.set_position(position);

        self
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn with_typeface(mut self, typeface: Option<Typeface>) -> Self {
        self.set_typeface(typeface);

        self
    }

    pub fn set_typeface(&mut self, typeface: Option<Typeface>) {
        self.typeface = typeface;
    }

    pub fn dispose(&self, sgl: &mut Sgl) {
        match self.text {
            TextType::Static(object_id) => sgl.delete_object(object_id),
            TextType::Dynamic(_) => {}
        }
    }

    pub fn translate_x(&mut self, x: f64) {
        self.position.x += x;
    }

    pub fn translate_y(&mut self, y: f64) {
        self.position.y += y;
    }
}
