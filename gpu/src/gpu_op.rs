use crate::{
    blend_mode::BlendMode, BufferType, FilterQuality, HintingLevel, PainterStyle, Pixel, Point,
    PointMode,
};

#[derive(Debug, Clone)]
pub enum GpuOp {
    Init {
        width: u64,
        height: u64,
    },
    GetPixel {
        x: f64,
        y: f64,
    },
    SetPainterColor {
        pixel: Pixel,
    },
    GetPainterColor,
    SetPainterStyle {
        style: PainterStyle,
    },
    GetPainterStyle,
    SetPainterBlendMode {
        mode: BlendMode,
    },
    GetPainterBlendMode,
    SetPainterFilterQuality {
        quality: FilterQuality,
    },
    GetPainterFilterQuality,
    SetPainterHintingLevel {
        level: HintingLevel,
    },
    GetPainterHintingLevel,
    SetPainterAutohinting {
        state: bool,
    },
    GetPainterAutohinting,
    SetPainterAntialiasing {
        state: bool,
    },
    GetPainterAntialiasing,
    SetPainterDithering {
        state: bool,
    },
    GetPainterDithering,
    CreateBuffer {
        ty: BufferType,
        address: usize,
        size: usize,
        length: usize,
    },
    DeleteBuffer {
        id: u64,
    },
    DeleteAllBuffers,
    DrawPixel {
        x: f64,
        y: f64,
    },
    DrawLine {
        a: Point,
        b: Point,
    },
    DrawRect {
        from: Point,
        width: f64,
        height: f64,
    },
    DrawRoundRect {
        from: Point,
        width: f64,
        height: f64,
        radius: Point,
    },
    DrawCircle {
        center: Point,
        radius: f64,
    },
    DrawOval {
        center: Point,
        hcenter: Point,
    },
    DrawPoints {
        buffer_id: u64,
        mode: PointMode,
    },
}

impl GpuOp {
    pub fn id(&self) -> u32 {
        match self {
            GpuOp::Init { .. } => 0x0,

            // Pixel ops
            GpuOp::GetPixel { .. } => 0x10,

            // Painter ops
            GpuOp::SetPainterColor { .. } => 0x100,
            GpuOp::GetPainterColor => 0x101,
            GpuOp::SetPainterStyle { .. } => 0x102,
            GpuOp::GetPainterStyle => 0x103,
            GpuOp::SetPainterBlendMode { .. } => 0x104,
            GpuOp::GetPainterBlendMode => 0x105,
            GpuOp::SetPainterFilterQuality { .. } => 0x106,
            GpuOp::GetPainterFilterQuality => 0x107,
            GpuOp::SetPainterHintingLevel { .. } => 0x108,
            GpuOp::GetPainterHintingLevel => 0x109,
            GpuOp::SetPainterAutohinting { .. } => 0x10A,
            GpuOp::GetPainterAutohinting => 0x10B,
            GpuOp::SetPainterAntialiasing { .. } => 0x10C,
            GpuOp::GetPainterAntialiasing => 0x10D,
            GpuOp::SetPainterDithering { .. } => 0x10E,
            GpuOp::GetPainterDithering => 0x10F,

            // Buffers ops
            GpuOp::CreateBuffer { .. } => 0x500,
            GpuOp::DeleteBuffer { .. } => 0x501,
            GpuOp::DeleteAllBuffers => 0x502,

            // Drawing ops
            GpuOp::DrawPixel { .. } => 0x1000,
            GpuOp::DrawLine { .. } => 0x1001,
            GpuOp::DrawRect { .. } => 0x1002,
            GpuOp::DrawRoundRect { .. } => 0x1003,
            GpuOp::DrawCircle { .. } => 0x1004,
            GpuOp::DrawOval { .. } => 0x1005,
            GpuOp::DrawPoints { .. } => 0x1006,
        }
    }
}
