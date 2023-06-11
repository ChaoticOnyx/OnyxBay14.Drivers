#![no_std]
#![feature(panic_info_message)]

use core::panic::Location;

use sgl::{
    gpu::{Boundable, BoundableExt, Color, MutPositionable, TextAlign},
    Sgl, Text,
};
use stack_string::StackString;

const TEXT_COLOR: Option<Color> = Some(Color::white());
const FONT_SIZE: f64 = 28.0;
const PANIC_TEXT: &str = "FATAL ERROR!";

pub fn bsod_panic(sgl: &mut Sgl, info: &core::panic::PanicInfo) -> ! {
    if let Some(reason) = info.payload().downcast_ref::<&str>() {
        bsod(sgl, Some(reason), info.location());
    } else if let Some(message) = info.message() {
        let mut msg = StackString::new();

        msg.format(format_args!("PANIC: {message}"));

        bsod(sgl, Some(msg.str()), info.location());
    } else {
        bsod(sgl, Some("PANIC"), info.location());
    }
}

pub fn bsod(sgl: &mut Sgl, reason: Option<&str>, location: Option<&Location>) -> ! {
    unsafe {
        sgl.fill_screen(Some(Color::blue()));

        let mut text_pos = sgl.bounds().center();

        let panic_text = Text::new_dynamic(PANIC_TEXT)
            .with_color(TEXT_COLOR)
            .with_size(Some(FONT_SIZE))
            .with_align(TextAlign::Center)
            .with_position(text_pos);

        sgl.draw_text(&panic_text);

        text_pos.y += FONT_SIZE - 14.0;

        if let Some(reason) = reason {
            let reason_text = Text::new_dynamic(reason)
                .with_color(TEXT_COLOR)
                .with_size(Some(FONT_SIZE - 14.0))
                .with_align(TextAlign::Center)
                .with_position(text_pos);

            sgl.draw_text(&reason_text);

            text_pos.y += FONT_SIZE - 14.0;
        }

        if let Some(location) = location {
            let mut msg = StackString::new();
            let mut file = location.file();

            if file.len() > 15 {
                file = &file[file.len() - 15..file.len()];
            }

            msg.format(format_args!(
                "LINE: {}, COLUMN: {}, FILE: ...{}",
                location.line(),
                location.column(),
                file
            ));

            let text = Text::new_dynamic(msg.str())
                .with_color(TEXT_COLOR)
                .with_size(Some(FONT_SIZE - 14.0))
                .with_align(TextAlign::Center)
                .with_position(text_pos);

            sgl.draw_text(&text);
        }

        sgl.flush();

        loop {
            riscv::asm::wfi();
        }
    }
}
