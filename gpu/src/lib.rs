#![no_std]

mod blend_mode;
mod color;
mod filter_quality;
mod gpu_argument;
mod gpu_error;
mod gpu_op;
mod hinting_level;
mod object;
mod painter_style;
mod point;
mod points_mode;
mod rect;
mod text_align;

pub use blend_mode::BlendMode;
pub use color::Color;
pub use filter_quality::FilterQuality;
pub use gpu_argument::GpuArgument;
pub use gpu_error::GpuError;
pub use gpu_op::GpuOp;
pub use hinting_level::HintingLevel;
pub use object::ObjectType;
pub use painter_style::PainterStyle;
pub use point::Point;
pub use points_mode::PointMode;
pub use rect::Rect;
pub use text_align::TextAlign;

use pci::PciDevice;

pub const DEVICE_ID: u16 = 0x66;

pub struct Gpu {
    pub device: PciDevice,
}

impl Gpu {
    pub unsafe fn flip_buffers(&mut self) {
        self.device.mmio.write_u8(0x1, 0x1);
    }

    pub unsafe fn width(&self) -> u32 {
        self.device.mmio.read_u32(0x1)
    }

    pub unsafe fn height(&self) -> u32 {
        self.device.mmio.read_u32(0x2)
    }

    pub unsafe fn arg(&self, arg: GpuArgument) -> f64 {
        self.device.mmio.read_f64(arg.offset())
    }

    pub unsafe fn set_arg(&mut self, arg: GpuArgument, value: f64) {
        self.device.mmio.write_f64(value, arg.offset());
    }

    pub unsafe fn call_op(&mut self, op: GpuOp) -> Result<f64, GpuError> {
        match op {
            GpuOp::Init { width, height } => {
                self.set_arg(GpuArgument::Arg0, width as f64);
                self.set_arg(GpuArgument::Arg1, height as f64);
            }
            GpuOp::GetPixel { x, y } => {
                self.set_arg(GpuArgument::Arg0, x);
                self.set_arg(GpuArgument::Arg1, y);
            }
            GpuOp::SetPainterColor { color } => {
                self.set_arg(GpuArgument::Arg0, color.r as f64);
                self.set_arg(GpuArgument::Arg1, color.g as f64);
                self.set_arg(GpuArgument::Arg2, color.b as f64);
                self.set_arg(GpuArgument::Arg3, color.a as f64);
            }
            GpuOp::GetPainterColor => {}
            GpuOp::SetPainterStyle { style } => {
                self.set_arg(GpuArgument::Arg0, style as i64 as f64);
            }
            GpuOp::GetPainterStyle => {}
            GpuOp::SetPainterBlendMode { mode } => {
                self.set_arg(GpuArgument::Arg0, mode as i64 as f64);
            }
            GpuOp::GetPainterBlendMode => {}
            GpuOp::SetPainterFilterQuality { quality } => {
                self.set_arg(GpuArgument::Arg0, quality as i64 as f64)
            }
            GpuOp::GetPainterFilterQuality => {}
            GpuOp::SetPainterHintingLevel { level } => {
                self.set_arg(GpuArgument::Arg0, level as i64 as f64)
            }
            GpuOp::GetPainterHintingLevel => {}
            GpuOp::SetPainterAutohinting { state } => {
                self.set_arg(GpuArgument::Arg0, state as i64 as f64)
            }
            GpuOp::GetPainterAutohinting => {}
            GpuOp::SetPainterAntialiasing { state } => {
                self.set_arg(GpuArgument::Arg0, state as i64 as f64)
            }
            GpuOp::GetPainterAntialiasing => {}
            GpuOp::SetPainterDithering { state } => {
                self.set_arg(GpuArgument::Arg0, state as i64 as f64)
            }
            GpuOp::GetPainterDithering => {}
            GpuOp::MesaureText { object_id } => self.set_arg(GpuArgument::Arg0, object_id as f64),
            GpuOp::SetPainterTypeface { object_id } => {
                self.set_arg(GpuArgument::Arg0, object_id.unwrap_or_default() as f64)
            }
            GpuOp::SetPainterTextSize { size } => self.set_arg(GpuArgument::Arg0, size),
            GpuOp::GetPainterTextSize => {}
            GpuOp::SetPainterTextScaleX { scale } => self.set_arg(GpuArgument::Arg0, scale),
            GpuOp::GetPainterTextScaleX => {}
            GpuOp::SetPainterTextSkewX { skew } => self.set_arg(GpuArgument::Arg0, skew),
            GpuOp::GetPainterTextSkewX => {}
            GpuOp::SetPainterTextAlign { align } => {
                self.set_arg(GpuArgument::Arg0, align as i64 as f64)
            }
            GpuOp::GetPainterTextAlign => {}
            GpuOp::SetPainterSubpixelText { state } => {
                self.set_arg(GpuArgument::Arg0, if state { 1f64 } else { 0f64 })
            }
            GpuOp::GetPainterSubpixelText => {}
            GpuOp::MesaureString { address, length } => {
                self.set_arg(GpuArgument::Arg0, address as f64);
                self.set_arg(GpuArgument::Arg1, length as f64);
            }
            GpuOp::CreateObject {
                ty,
                address,
                size,
                length,
            } => {
                self.set_arg(GpuArgument::Arg0, ty as i64 as f64);
                self.set_arg(GpuArgument::Arg1, address as f64);
                self.set_arg(GpuArgument::Arg2, size as f64);
                self.set_arg(GpuArgument::Arg3, length as f64);
            }
            GpuOp::DeleteObject { object_id } => {
                self.set_arg(GpuArgument::Arg0, object_id as f64);
            }
            GpuOp::DeleteAllObjects => {}
            GpuOp::CreateImageObject {
                width,
                height,
                address,
            } => {
                self.set_arg(GpuArgument::Arg0, width as f64);
                self.set_arg(GpuArgument::Arg1, height as f64);
                self.set_arg(GpuArgument::Arg2, address as f64);
            }
            GpuOp::CreateSurfaceObject { width, height } => {
                self.set_arg(GpuArgument::Arg0, width as f64);
                self.set_arg(GpuArgument::Arg1, height as f64);
            }
            GpuOp::SwitchSurface { object_id } => {
                self.set_arg(GpuArgument::Arg0, object_id as f64);
            }
            GpuOp::DrawPixel { x, y } => {
                self.set_arg(GpuArgument::Arg0, x);
                self.set_arg(GpuArgument::Arg1, y);
            }
            GpuOp::DrawLine { a, b } => {
                self.set_arg(GpuArgument::Arg0, a.x);
                self.set_arg(GpuArgument::Arg1, a.y);
                self.set_arg(GpuArgument::Arg2, b.x);
                self.set_arg(GpuArgument::Arg3, b.y);
            }
            GpuOp::DrawRect {
                from,
                width,
                height,
            } => {
                self.set_arg(GpuArgument::Arg0, from.x);
                self.set_arg(GpuArgument::Arg1, from.y);
                self.set_arg(GpuArgument::Arg2, width);
                self.set_arg(GpuArgument::Arg3, height);
            }
            GpuOp::DrawRoundRect {
                from,
                width,
                height,
                radius,
            } => {
                self.set_arg(GpuArgument::Arg0, from.x);
                self.set_arg(GpuArgument::Arg1, from.y);
                self.set_arg(GpuArgument::Arg2, width);
                self.set_arg(GpuArgument::Arg3, height);
                self.set_arg(GpuArgument::Arg4, radius.x);
                self.set_arg(GpuArgument::Arg5, radius.y);
            }
            GpuOp::DrawCircle { center, radius } => {
                self.set_arg(GpuArgument::Arg0, center.x);
                self.set_arg(GpuArgument::Arg1, center.y);
                self.set_arg(GpuArgument::Arg2, radius);
            }
            GpuOp::DrawOval { center, hcenter } => {
                self.set_arg(GpuArgument::Arg0, center.x);
                self.set_arg(GpuArgument::Arg1, center.y);
                self.set_arg(GpuArgument::Arg2, hcenter.x);
                self.set_arg(GpuArgument::Arg3, hcenter.y);
            }
            GpuOp::DrawPoints { object_id, mode } => {
                self.set_arg(GpuArgument::Arg0, object_id as f64);
                self.set_arg(GpuArgument::Arg1, mode as u64 as f64);
            }
            GpuOp::DrawText { object_id, point } => {
                self.set_arg(GpuArgument::Arg0, object_id as f64);
                self.set_arg(GpuArgument::Arg1, point.x);
                self.set_arg(GpuArgument::Arg2, point.y);
            }
            GpuOp::DrawImage { object_id, point } => {
                self.set_arg(GpuArgument::Arg0, object_id as f64);
                self.set_arg(GpuArgument::Arg1, point.x);
                self.set_arg(GpuArgument::Arg2, point.y);
            }
            GpuOp::DrawImageRect { object_id, dst } => {
                self.set_arg(GpuArgument::Arg0, object_id as f64);
                self.set_arg(GpuArgument::Arg1, dst.left);
                self.set_arg(GpuArgument::Arg2, dst.top);
                self.set_arg(GpuArgument::Arg3, dst.right);
                self.set_arg(GpuArgument::Arg4, dst.bottom);
            }
            GpuOp::DrawImageRectSrc {
                object_id,
                src,
                dst,
            } => {
                self.set_arg(GpuArgument::Arg0, object_id as f64);
                self.set_arg(GpuArgument::Arg1, src.left);
                self.set_arg(GpuArgument::Arg2, src.top);
                self.set_arg(GpuArgument::Arg3, src.right);
                self.set_arg(GpuArgument::Arg4, src.bottom);
                self.set_arg(GpuArgument::Arg5, dst.left);
                self.set_arg(GpuArgument::Arg6, dst.top);
                self.set_arg(GpuArgument::Arg7, dst.right);
                self.set_arg(GpuArgument::Arg8, dst.bottom);
            }
            GpuOp::DrawString {
                point,
                address,
                length,
            } => {
                self.set_arg(GpuArgument::Arg0, point.x);
                self.set_arg(GpuArgument::Arg1, point.y);
                self.set_arg(GpuArgument::Arg2, address as f64);
                self.set_arg(GpuArgument::Arg3, length as f64);
            }
        }

        self.device.mmio.write_u32(op.id(), 0x0);
        let ret = self.device.mmio.read_f64(0x0);

        if (ret as i64) < 0 {
            Err(GpuError::from(ret as i64))
        } else {
            Ok(ret)
        }
    }
}

impl From<PciDevice> for Gpu {
    fn from(value: PciDevice) -> Self {
        Self { device: value }
    }
}
