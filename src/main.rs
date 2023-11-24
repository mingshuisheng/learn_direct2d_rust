use std::iter::once;

use windows::core::w;
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_NO_SNAP;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F};
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_ULTRA_BLACK, DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWriteCreateFactory, IDWriteFactory2};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();
    let write_factory: IDWriteFactory2 = unsafe {
        DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap()
    };

    let width = 640;
    let height = 480;

    graphic.draw_and_save((width, height), w!("1.png"), |ctx| unsafe {
        let length = 80;
        let background = from_str("#9ac5e5");
        ctx.Clear(Some(&background));
        let line = from_str("#e5c6c3");
        let solid_brush = ctx.CreateSolidColorBrush(&line, None).unwrap();

        let stroke_width = 5.0;

        for i in (0..width).step_by(length) {
            if i == 0 {
                continue;
            }
            ctx.DrawLine(
                D2D_POINT_2F {
                    x: i as f32,
                    y: 0.0,
                },
                D2D_POINT_2F {
                    x: i as f32,
                    y: height as f32,
                },
                &solid_brush,
                stroke_width,
                None,
            );
        }

        for i in (0..height).step_by(length) {
            if i == 0 {
                continue;
            }
            ctx.DrawLine(
                D2D_POINT_2F {
                    x: 0.0,
                    y: i as f32,
                },
                D2D_POINT_2F {
                    x: width as f32,
                    y: i as f32,
                },
                &solid_brush,
                stroke_width,
                None,
            );
        }

        let font_color = from_str("#C98C9A");
        let solid_brush = ctx.CreateSolidColorBrush(&font_color, None).unwrap();
        let font_family = w!("Courier New");
        let text_format = write_factory.CreateTextFormat(
            font_family,
            None,
            DWRITE_FONT_WEIGHT_ULTRA_BLACK,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            (length as f32) * 0.6,
            w!(""),
        ).unwrap();

        let total_col = width / length as u32;
        let total_row = height / length as u32;
        println!("{total_col}, {total_row}");

        for i in 0..total_row {
            for j in 0..total_col {
                let count = i * total_col + j;
                let text = format!("{count:0>2}").to_string().encode_utf16().chain(once(0)).collect::<Vec<u16>>();
                let text_layout = write_factory.CreateTextLayout(text.as_slice(), &text_format, f32::MAX, f32::MAX).unwrap();

                text_layout.SetMaxHeight(length as f32).unwrap();
                text_layout.SetMaxWidth(length as f32).unwrap();

                text_layout.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER).unwrap();
                text_layout.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER).unwrap();
                let position = D2D_POINT_2F {
                    x: j as f32 * length as f32,
                    y: i as f32 * length as f32,
                };

                ctx.DrawTextLayout(position, &text_layout, &solid_brush, D2D1_DRAW_TEXT_OPTIONS_NO_SNAP);
            }
        }
    });
}

fn from_str(s: &str) -> D2D1_COLOR_F {
    let s = s.trim_start_matches('#');
    let (r, g, b, a) = {
        if s.len() == 3 {
            (to_u8(&s[0..1].repeat(2)), to_u8(&s[1..2].repeat(2)), to_u8(&s[2..3].repeat(2)), 255)
        } else if s.len() == 4 {
            (to_u8(&s[0..1].repeat(2)), to_u8(&s[1..2].repeat(2)), to_u8(&s[2..3].repeat(2)), to_u8(&s[3..4].repeat(2)))
        } else if s.len() == 6 {
            (to_u8(&s[0..2]), to_u8(&s[2..4]), to_u8(&s[4..6]), 255)
        } else if s.len() == 8 {
            (to_u8(&s[0..2]), to_u8(&s[2..4]), to_u8(&s[4..6]), to_u8(&s[6..8]))
        } else {
            (0, 0, 0, 255)
        }
    };

    D2D1_COLOR_F {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: a as f32 / 255.0,
    }
}

fn to_u8(s: &str) -> u8 {
    u8::from_str_radix(s, 16).unwrap()
}