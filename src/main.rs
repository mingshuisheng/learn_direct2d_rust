use std::iter::once;

use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_NO_SNAP;
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_ULTRA_BLACK, DWRITE_MEASURING_MODE_NATURAL, DWriteCreateFactory, IDWriteFactory2};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();
    let write_factory: IDWriteFactory2 = unsafe {
        DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap()
    };

    graphic.draw_and_save((640, 480), w!("output.png"), |ctx| unsafe {
        ctx.Clear(Some(&D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }));

        let text = "Hello,World!".to_string();
        let text_vec = text.encode_utf16().chain(once(0)).collect::<Vec<u16>>();
        let font_family = w!("微软雅黑");
        let font_size = 40.0;
        let text_format = write_factory.CreateTextFormat(
            font_family,
            None,
            DWRITE_FONT_WEIGHT_ULTRA_BLACK,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            font_size,
            w!(""),
        ).unwrap();

        let rect = D2D_RECT_F {
            left: 0.0,
            top: 0.0,
            right: 640.0,
            bottom: 480.0,
        };

        let brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }, None).unwrap();

        ctx.DrawText(&text_vec, &text_format, &rect, &brush, D2D1_DRAW_TEXT_OPTIONS_NO_SNAP, DWRITE_MEASURING_MODE_NATURAL);
    });
}