use gpu::{Color, MutPositionable, Point, Positionable, TextAlign};

use crate::Typeface;

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
pub enum TextType<T>
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

    pub(crate) fn new_from_id(object_id: u64) -> Self {
        Self {
            text: TextType::Static(object_id),
            color: None,
            size: None,
            align: TextAlign::Left,
            position: Point::zero(),
            typeface: None,
        }
    }

    pub fn typeface(&self) -> Option<Typeface> {
        self.typeface
    }

    pub fn text(&self) -> &TextType<T> {
        &self.text
    }

    pub fn size(&self) -> Option<f64> {
        self.size
    }

    pub fn color(&self) -> Option<Color> {
        self.color
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

    pub fn align(&self) -> TextAlign {
        self.align
    }

    pub fn with_typeface(mut self, typeface: Option<Typeface>) -> Self {
        self.set_typeface(typeface);

        self
    }

    pub fn set_typeface(&mut self, typeface: Option<Typeface>) {
        self.typeface = typeface;
    }
}

impl<T> Positionable for Text<T>
where
    T: AsRef<str>,
{
    fn position(&self) -> Point {
        self.position
    }
}

impl<T> MutPositionable for Text<T>
where
    T: AsRef<str>,
{
    fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    fn with_position(mut self, position: Point) -> Self {
        self.set_position(position);

        self
    }
}
