use std::iter::once;

use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::{D2D1_DRAW_TEXT_OPTIONS_CLIP, D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING, D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT, D2D1_DRAW_TEXT_OPTIONS_NO_SNAP, D2D1_DRAW_TEXT_OPTIONS_NONE};
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_ULTRA_BLACK, DWRITE_MEASURING_MODE_NATURAL, DWRITE_WORD_WRAPPING_NO_WRAP, DWriteCreateFactory, IDWriteFactory2};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();
    let write_factory: IDWriteFactory2 = unsafe {
        DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap()
    };

    // let font_family = w!("微软雅黑");
    let font_family = w!("Segoe UI Emoji");
    let font_size = 70.0;

    let rect = D2D_RECT_F {
        left: 0.0,
        top: 0.0,
        right: 640.0 / 3.0,
        bottom: 480.0 / 3.0,
    };

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

    let border_color =  D2D1_COLOR_F {
        r: 0.0,
        g: 0.8,
        b: 0.0,
        a: 1.0,
     };

    let text = "😂Hello, world!";
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

    graphic.draw_and_save((640, 480), w!("D2D1_DRAW_TEXT_OPTIONS_NONE.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        ctx.DrawText(&text_vec, &text_format, &rect, &brush, D2D1_DRAW_TEXT_OPTIONS_NONE, DWRITE_MEASURING_MODE_NATURAL);

        //border
        let brush = ctx.CreateSolidColorBrush(&border_color, None).unwrap();
        ctx.DrawRectangle(&rect, &brush, 2.0, None);
    });

    graphic.draw_and_save((640, 480), w!("D2D1_DRAW_TEXT_OPTIONS_CLIP.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        ctx.DrawText(&text_vec, &text_format, &rect, &brush, D2D1_DRAW_TEXT_OPTIONS_CLIP, DWRITE_MEASURING_MODE_NATURAL);

        //border
        let brush = ctx.CreateSolidColorBrush(&border_color, None).unwrap();
        ctx.DrawRectangle(&rect, &brush, 2.0, None);
    });

    graphic.draw_and_save((640, 480), w!("D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        ctx.DrawText(&text_vec, &text_format, &rect, &brush, D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT, DWRITE_MEASURING_MODE_NATURAL);

        //border
        let brush = ctx.CreateSolidColorBrush(&border_color, None).unwrap();
        ctx.DrawRectangle(&rect, &brush, 2.0, None);
    });


    graphic.draw_and_save((640, 480), w!("D2D1_DRAW_TEXT_OPTIONS_NO_SNAP.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        ctx.DrawText(&text_vec, &text_format, &rect, &brush, D2D1_DRAW_TEXT_OPTIONS_NO_SNAP, DWRITE_MEASURING_MODE_NATURAL);

        //border
        let brush = ctx.CreateSolidColorBrush(&border_color, None).unwrap();
        ctx.DrawRectangle(&rect, &brush, 2.0, None);
    });


    graphic.draw_and_save((640, 480), w!("D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        ctx.DrawText(&text_vec, &text_format, &rect, &brush, D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING, DWRITE_MEASURING_MODE_NATURAL);

        //border
        let brush = ctx.CreateSolidColorBrush(&border_color, None).unwrap();
        ctx.DrawRectangle(&rect, &brush, 2.0, None);
    });

}