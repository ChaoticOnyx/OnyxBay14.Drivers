#![no_std]

use gpu::{Boundable, Color, Gpu, GpuOp, Point, Positionable, Rect, TextAlign};
use pci::PciBus;
use screen::Screen;

mod image;
mod text;
mod typeface;

pub use image::Image;
pub use text::{Text, TextType};
pub use typeface::Typeface;

pub use gpu;

static mut SGL: Option<Sgl> = None;

pub struct Sgl {
    gpu: Gpu,
    screen: Screen,
    bounds: Rect,
    font_size: Option<f64>,
    color: Option<Color>,
}

impl Sgl {
    pub fn init(mut self) {
        unsafe {
            self.gpu
                .call_op(GpuOp::Init {
                    width: self.bounds.width() as u64,
                    height: self.bounds.height() as u64,
                })
                .unwrap();

            self.screen.connect(self.gpu.device.mmio.address);

            while !self.screen.is_connected() {}

            SGL.replace(self);
        }
    }

    pub fn create_typeface(&mut self, data: &'static [u8]) -> Typeface {
        let object_id = unsafe {
            self.gpu
                .call_op(GpuOp::CreateTypefaceObject {
                    address: data.as_ptr() as usize,
                    size: data.len(),
                })
                .unwrap() as u64
        };

        Typeface::new_from_id(object_id)
    }

    fn delete_object(&mut self, object_id: u64) {
        unsafe {
            self.gpu.call_op(GpuOp::DeleteObject { object_id }).unwrap();
        }
    }

    pub fn delete_image(&mut self, image: Image) {
        self.delete_object(image.id());
    }

    pub fn delete_text(&mut self, text: Text<impl AsRef<str>>) {
        match text.text() {
            TextType::Static(object_id) => {
                self.delete_object(*object_id);
            }
            TextType::Dynamic(_) => {}
        }
    }

    pub fn set_painter_color(&mut self, color: Color) {
        unsafe {
            self.gpu.call_op(GpuOp::SetPainterColor { color }).unwrap();
        }

        self.color.replace(color);
    }

    pub fn get_painter_color(&mut self) -> Color {
        if let Some(color) = self.color {
            return color;
        }

        self.color
            .replace(unsafe { self.gpu.call_op(GpuOp::GetPainterColor).unwrap().into() });
        self.color.unwrap()
    }

    pub fn swap_painter_color(&mut self, new_color: Color) -> Color {
        let old_color = self.get_painter_color();
        self.set_painter_color(new_color);

        old_color
    }

    pub fn swap_text_size(&mut self, new_size: f64) -> f64 {
        let old_size = self.get_painter_text_size();
        self.set_painter_text_size(new_size);

        old_size
    }

    pub fn get_painter_text_size(&mut self) -> f64 {
        if let Some(size) = self.font_size {
            return size;
        }

        self.font_size
            .replace(unsafe { self.gpu.call_op(GpuOp::GetPainterTextSize).unwrap() });
        self.font_size.unwrap()
    }

    pub fn set_painter_text_size(&mut self, size: f64) {
        unsafe {
            self.gpu
                .call_op(GpuOp::SetPainterTextSize { size })
                .unwrap();
        }

        self.font_size.replace(size);
    }

    pub fn set_painter_typeface(&mut self, typeface: Option<Typeface>) {
        unsafe {
            self.gpu
                .call_op(GpuOp::SetPainterTypeface {
                    object_id: typeface.map(|typeface| typeface.id()),
                })
                .unwrap();
        }
    }

    pub fn get_painter_typeface(&mut self) -> u64 {
        unsafe { self.gpu.call_op(GpuOp::GetPainterTypeface).unwrap() as u64 }
    }

    pub fn swap_painter_typeface(&mut self, typeface: Option<Typeface>) -> Option<Typeface> {
        let old_typeface = self.get_painter_typeface();

        self.set_painter_typeface(typeface);

        if old_typeface == 0 {
            None
        } else {
            Some(Typeface::new_from_id(old_typeface))
        }
    }

    pub fn fill_screen(&mut self, color: Option<Color>) {
        unsafe {
            let mut old_color: Option<Color> = None;

            if let Some(color) = color {
                old_color = Some(self.swap_painter_color(color));
            }

            self.gpu
                .call_op(GpuOp::DrawRect {
                    from: Point::new(0.0, 0.0),
                    width: self.bounds.width(),
                    height: self.bounds.height(),
                })
                .unwrap();

            if let Some(old_color) = old_color {
                self.set_painter_color(old_color)
            }
        }
    }

    pub fn draw_text(&mut self, text: &Text<impl AsRef<str>>) {
        let mut position = text.position();
        let mut old_color: Option<Color> = None;
        let mut old_size: Option<f64> = None;
        let mut old_typeface: Option<Option<Typeface>> = None;
        let old_align = self.swap_painter_text_align(text.align());

        if let Some(color) = text.color() {
            old_color = Some(self.swap_painter_color(color));
        }

        if let Some(size) = text.size() {
            old_size = Some(self.swap_text_size(size));
        }

        if let Some(typeface) = text.typeface() {
            old_typeface = Some(self.swap_painter_typeface(Some(typeface)));
        }

        let text_size = old_size.unwrap_or_else(|| self.get_painter_text_size());
        position.y += text_size;

        match text.text() {
            TextType::Static(object_id) => unsafe {
                self.gpu
                    .call_op(GpuOp::DrawText {
                        object_id: *object_id,
                        position,
                    })
                    .unwrap();
            },
            TextType::Dynamic(ref text) => unsafe {
                let text = text.as_ref();
                let address = text as *const str as *const u8 as usize;

                self.gpu
                    .call_op(GpuOp::DrawString {
                        position,
                        address,
                        length: text.as_bytes().len(),
                    })
                    .unwrap();
            },
        };

        if let Some(old_color) = old_color {
            self.set_painter_color(old_color);
        }

        if let Some(old_size) = old_size {
            self.set_painter_text_size(old_size);
        }

        if let Some(old_typeface) = old_typeface {
            self.set_painter_typeface(old_typeface)
        }

        self.set_painter_text_align(old_align);
    }

    pub fn create_static_text<T>(&mut self, text: T) -> Text<T>
    where
        T: AsRef<str>,
    {
        let text = text.as_ref();
        let object_id = unsafe {
            self.gpu
                .call_op(GpuOp::CreateTextObject {
                    address: text.as_ptr() as usize,
                    size: text.len(),
                })
                .unwrap() as u64
        };

        Text::new_from_id(object_id)
    }

    pub fn measure_text_bounds(&mut self, text: &Text<impl AsRef<str>>) -> Rect {
        let mut old_typeface: Option<Option<Typeface>> = None;

        if let Some(typeface) = text.typeface() {
            old_typeface = Some(self.swap_painter_typeface(Some(typeface)));
        }

        let result = match text.text() {
            TextType::Static(object_id) => unsafe {
                let width = self
                    .gpu
                    .call_op(GpuOp::MeasureTextWidth {
                        object_id: *object_id,
                    })
                    .unwrap();

                let height = self
                    .gpu
                    .call_op(GpuOp::MeasureTextHeight {
                        object_id: *object_id,
                    })
                    .unwrap();

                Rect::new_from_zero(width, height)
            },
            TextType::Dynamic(text) => unsafe {
                let text = text.as_ref();
                let address = text as *const str as *const u8 as usize;

                let width = self
                    .gpu
                    .call_op(GpuOp::MeasureStringWidth {
                        address,
                        length: text.as_bytes().len(),
                    })
                    .unwrap();

                let height = self
                    .gpu
                    .call_op(GpuOp::MeasureStringHeight {
                        address,
                        length: text.as_bytes().len(),
                    })
                    .unwrap();

                Rect::new_from_zero(width, height)
            },
        };

        if let Some(old_typeface) = old_typeface {
            self.set_painter_typeface(old_typeface);
        }

        result
    }

    pub fn set_painter_text_align(&mut self, align: TextAlign) {
        unsafe {
            self.gpu
                .call_op(GpuOp::SetPainterTextAlign { align })
                .unwrap();
        }
    }

    pub fn get_painter_text_align(&mut self) -> TextAlign {
        unsafe { (self.gpu.call_op(GpuOp::GetPainterTextAlign).unwrap() as u8).into() }
    }

    pub fn swap_painter_text_align(&mut self, align: TextAlign) -> TextAlign {
        let old_align = self.get_painter_text_align();
        self.set_painter_text_align(align);

        old_align
    }

    pub fn create_image(&mut self, data: &[u8], width: u64, height: u64) -> Image {
        let id = unsafe {
            self.gpu
                .call_op(GpuOp::CreateImageObject {
                    width,
                    height,
                    address: data.as_ptr() as usize,
                })
                .unwrap() as u64
        };

        Image::new_from_id(Rect::new_from_zero(width as f64, height as f64), id)
    }

    pub fn draw_image(&mut self, image: &Image, position: Point) {
        unsafe {
            self.gpu
                .call_op(GpuOp::DrawImage {
                    object_id: image.id(),
                    position,
                })
                .unwrap();
        }
    }

    pub fn draw_image_rect(&mut self, image: &Image, dst: Rect) {
        unsafe {
            self.gpu
                .call_op(GpuOp::DrawImageRect {
                    object_id: image.id(),
                    dst,
                })
                .unwrap();
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            self.gpu.flush();
        }
    }

    pub fn mut_get() -> &'static mut Option<Sgl> {
        unsafe { &mut SGL }
    }

    pub fn get() -> &'static Option<Sgl> {
        unsafe { &SGL }
    }
}

impl Positionable for Sgl {
    #[inline]
    fn position(&self) -> Point {
        self.bounds.position()
    }
}

impl Boundable for Sgl {
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

impl Default for Sgl {
    fn default() -> Self {
        unsafe {
            let pci = PciBus::default();
            let screen = pci.find_by_id(screen::DEVICE_ID).map(Screen::from).unwrap();
            let gpu = pci.find_by_id(gpu::DEVICE_ID).map(Gpu::from).unwrap();
            let height = screen.height() as f64;
            let width = screen.width() as f64;

            Self {
                gpu,
                screen,
                bounds: Rect::new_from_zero(width, height),
                font_size: None,
                color: None,
            }
        }
    }
}
