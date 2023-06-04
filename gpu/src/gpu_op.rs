use crate::{
    blend_mode::BlendMode, Color, FilterQuality, HintingLevel, ObjectType, PainterStyle, Point,
    PointMode, Rect, TextAlign,
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
        color: Color,
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
    MesaureText {
        object_id: u64,
    },
    SetPainterTypeface {
        object_id: Option<u64>,
    },
    SetPainterTextSize {
        size: f64,
    },
    GetPainterTextSize,
    SetPainterTextScaleX {
        scale: f64,
    },
    GetPainterTextScaleX,
    SetPainterTextSkewX {
        skew: f64,
    },
    GetPainterTextSkewX,
    SetPainterTextAlign {
        align: TextAlign,
    },
    GetPainterTextAlign,
    SetPainterSubpixelText {
        state: bool,
    },
    GetPainterSubpixelText,
    MesaureString {
        address: usize,
        length: usize,
    },
    CreateObject {
        ty: ObjectType,
        address: usize,
        size: usize,
        length: usize,
    },
    DeleteObject {
        object_id: u64,
    },
    DeleteAllObjects,
    CreateImageObject {
        width: u64,
        height: u64,
        address: usize,
    },
    CreateSurfaceObject {
        width: u64,
        height: u64,
    },
    SwitchSurface {
        object_id: u64,
    },
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
        object_id: u64,
        mode: PointMode,
    },
    DrawText {
        object_id: u64,
        position: Point,
    },
    DrawImage {
        object_id: u64,
        position: Point,
    },
    DrawImageRect {
        object_id: u64,
        dst: Rect,
    },
    DrawImageRectSrc {
        object_id: u64,
        src: Rect,
        dst: Rect,
    },
    DrawString {
        position: Point,
        address: usize,
        length: usize,
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
            GpuOp::MesaureText { .. } => 0x110,
            GpuOp::SetPainterTypeface { .. } => 0x111,
            GpuOp::SetPainterTextSize { .. } => 0x112,
            GpuOp::GetPainterTextSize => 0x113,
            GpuOp::SetPainterTextScaleX { .. } => 0x114,
            GpuOp::GetPainterTextScaleX => 0x115,
            GpuOp::SetPainterTextSkewX { .. } => 0x116,
            GpuOp::GetPainterTextSkewX => 0x117,
            GpuOp::SetPainterTextAlign { .. } => 0x118,
            GpuOp::GetPainterTextAlign => 0x119,
            GpuOp::SetPainterSubpixelText { .. } => 0x11A,
            GpuOp::GetPainterSubpixelText => 0x11B,
            GpuOp::MesaureString { .. } => 0x11C,

            // Objects ops
            GpuOp::CreateObject { .. } => 0x500,
            GpuOp::DeleteObject { .. } => 0x501,
            GpuOp::DeleteAllObjects => 0x502,
            GpuOp::CreateImageObject { .. } => 0x503,
            GpuOp::CreateSurfaceObject { .. } => 0x504,
            GpuOp::SwitchSurface { .. } => 0x505,

            // Drawing ops
            GpuOp::DrawPixel { .. } => 0x1000,
            GpuOp::DrawLine { .. } => 0x1001,
            GpuOp::DrawRect { .. } => 0x1002,
            GpuOp::DrawRoundRect { .. } => 0x1003,
            GpuOp::DrawCircle { .. } => 0x1004,
            GpuOp::DrawOval { .. } => 0x1005,
            GpuOp::DrawPoints { .. } => 0x1006,
            GpuOp::DrawText { .. } => 0x1007,
            GpuOp::DrawImage { .. } => 0x1008,
            GpuOp::DrawImageRect { .. } => 0x1009,
            GpuOp::DrawImageRectSrc { .. } => 0x100A,
            GpuOp::DrawString { .. } => 0x100B,
        }
    }
}
