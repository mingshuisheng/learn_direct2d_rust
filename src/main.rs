use std::iter::once;

use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F};
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_CLIP;
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_ULTRA_BLACK, DWRITE_WORD_WRAPPING_NO_WRAP, DWriteCreateFactory, IDWriteFactory2};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();
    let write_factory: IDWriteFactory2 = unsafe {
        DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap()
    };
    let font_family = w!("微软雅黑");
    let font_size = 40.0;
    let graphic_color = D2D1_COLOR_F {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    };
    let text_color = D2D1_COLOR_F {
        r: 0.8,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    let text = "Hello, world!";
    let text_vec = text.encode_utf16().chain(once(0)).collect::<Vec<u16>>();
    let text_format = unsafe {
        write_factory.CreateTextFormat(
            font_family,
            None,
            DWRITE_FONT_WEIGHT_ULTRA_BLACK,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            font_size,
            w!(""),
        ).unwrap()
    };

    let text_layout = unsafe {
        write_factory.CreateTextLayout(&text_vec, &text_format, 640.0, 480.0).unwrap()
    };

    let origin = D2D_POINT_2F {
        x: 0.0,
        y: 0.0,
    };

    graphic.draw_and_save((640, 480), w!("draw_text_layout.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        ctx.DrawTextLayout(origin, &text_layout, &brush, D2D1_DRAW_TEXT_OPTIONS_CLIP);
    });
}