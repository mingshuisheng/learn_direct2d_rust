use std::iter::once;

use windows::core::{PCWSTR, w};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_NO_SNAP;
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP, DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT, DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT, DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_ULTRA_BLACK, DWRITE_LINE_SPACING_METHOD_DEFAULT, DWRITE_LINE_SPACING_METHOD_PROPORTIONAL, DWRITE_LINE_SPACING_METHOD_UNIFORM, DWRITE_MEASURING_MODE_NATURAL, DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_PARAGRAPH_ALIGNMENT_FAR, DWRITE_PARAGRAPH_ALIGNMENT_NEAR, DWRITE_READING_DIRECTION_BOTTOM_TO_TOP, DWRITE_READING_DIRECTION_LEFT_TO_RIGHT, DWRITE_READING_DIRECTION_RIGHT_TO_LEFT, DWRITE_READING_DIRECTION_TOP_TO_BOTTOM, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_JUSTIFIED, DWRITE_TEXT_ALIGNMENT_LEADING, DWRITE_TEXT_ALIGNMENT_TRAILING, DWRITE_TRIMMING, DWRITE_TRIMMING_GRANULARITY, DWRITE_TRIMMING_GRANULARITY_CHARACTER, DWRITE_TRIMMING_GRANULARITY_NONE, DWRITE_TRIMMING_GRANULARITY_WORD, DWRITE_WORD_WRAPPING_CHARACTER, DWRITE_WORD_WRAPPING_EMERGENCY_BREAK, DWRITE_WORD_WRAPPING_NO_WRAP, DWRITE_WORD_WRAPPING_WHOLE_WORD, DWRITE_WORD_WRAPPING_WRAP, DWriteCreateFactory, IDWriteFactory2, IDWriteTextFormat};

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

    let rect = D2D_RECT_F {
        left: 0.0,
        top: 0.0,
        right: 640.0,
        bottom: 480.0,
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

    let set_text_format = |filename: PCWSTR, text: &str, f: fn(&IDWriteTextFormat, &IDWriteFactory2)| {
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
        graphic.draw_and_save((640, 480), filename, |ctx| unsafe {
            ctx.Clear(Some(&graphic_color));
            f(&text_format, &write_factory);
            let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
            ctx.DrawText(&text_vec, &text_format, &rect, &brush, D2D1_DRAW_TEXT_OPTIONS_NO_SNAP, DWRITE_MEASURING_MODE_NATURAL);
        });
    };

    // text alignment
    let text = "Hello,world!Hello,world!Hello,world!Hello,world!Hello,world!Hello,world!Hello,world!";
    set_text_format(w!("DWRITE_TEXT_ALIGNMENT_LEADING.png"), text, |text_format, _| unsafe {
        text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING).unwrap();
    });
    set_text_format(w!("DWRITE_TEXT_ALIGNMENT_TRAILING.png"), text, |text_format, _| unsafe {
        text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_TRAILING).unwrap();
    });
    set_text_format(w!("DWRITE_TEXT_ALIGNMENT_CENTER.png"), text, |text_format, _| unsafe {
        text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER).unwrap();
    });
    set_text_format(w!("DWRITE_TEXT_ALIGNMENT_JUSTIFIED.png"), text, |text_format, _| unsafe {
        text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_JUSTIFIED).unwrap();
    });

    //text paragraph alignment
    set_text_format(w!("DWRITE_PARAGRAPH_ALIGNMENT_NEAR.png"), text, |text_format, _| unsafe {
        text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_NEAR).unwrap();
    });
    set_text_format(w!("DWRITE_PARAGRAPH_ALIGNMENT_FAR.png"), text, |text_format, _| unsafe {
        text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_FAR).unwrap();
    });
    set_text_format(w!("DWRITE_PARAGRAPH_ALIGNMENT_CENTER.png"), text, |text_format, _| unsafe {
        text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER).unwrap();
    });

    // word wrapping
    let text = "Hello Helllllllo Woooooooooooooooooooooorld";
    set_text_format(w!("DWRITE_WORD_WRAPPING_WRAP.png"), text, |text_format, _| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_WRAP).unwrap();
    });
    set_text_format(w!("DWRITE_WORD_WRAPPING_NO_WRAP.png"), text, |text_format, _| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
    });
    set_text_format(w!("DWRITE_WORD_WRAPPING_EMERGENCY_BREAK.png"), text, |text_format, _| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_EMERGENCY_BREAK).unwrap();
    });
    set_text_format(w!("DWRITE_WORD_WRAPPING_WHOLE_WORD.png"), text, |text_format, _| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_WHOLE_WORD).unwrap();
    });
    set_text_format(w!("DWRITE_WORD_WRAPPING_CHARACTER.png"), text, |text_format, _| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_CHARACTER).unwrap();
    });

    // reading direction
    let text = "Hello, world!";
    set_text_format(w!("DWRITE_READING_DIRECTION_LEFT_TO_RIGHT.png"), text, |text_format, _| unsafe {
        text_format.SetReadingDirection(DWRITE_READING_DIRECTION_LEFT_TO_RIGHT).unwrap();
    });
    set_text_format(w!("DWRITE_READING_DIRECTION_RIGHT_TO_LEFT.png"), text, |text_format, _| unsafe {
        text_format.SetReadingDirection(DWRITE_READING_DIRECTION_RIGHT_TO_LEFT).unwrap();
    });
    set_text_format(w!("DWRITE_READING_DIRECTION_TOP_TO_BOTTOM.png"), text, |text_format, _| unsafe {
        text_format.SetReadingDirection(DWRITE_READING_DIRECTION_TOP_TO_BOTTOM).unwrap();
        text_format.SetFlowDirection(DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT).unwrap();
    });
    set_text_format(w!("DWRITE_READING_DIRECTION_BOTTOM_TO_TOP.png"), text, |text_format, _| unsafe {
        text_format.SetReadingDirection(DWRITE_READING_DIRECTION_BOTTOM_TO_TOP).unwrap();
        text_format.SetFlowDirection(DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT).unwrap();
    });

    // flow direction
    let text = "Hello, world!";
    set_text_format(w!("DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM.png"), text, |text_format, _| unsafe {
        text_format.SetFlowDirection(DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM).unwrap();
    });
    set_text_format(w!("DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP.png"), text, |text_format, _| unsafe {
        text_format.SetFlowDirection(DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP).unwrap();
    });
    set_text_format(w!("DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT.png"), text, |text_format, _| unsafe {
        text_format.SetReadingDirection(DWRITE_READING_DIRECTION_TOP_TO_BOTTOM).unwrap();
        text_format.SetFlowDirection(DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT).unwrap();
    });
    set_text_format(w!("DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT.png"), text, |text_format, _| unsafe {
        text_format.SetReadingDirection(DWRITE_READING_DIRECTION_TOP_TO_BOTTOM).unwrap();
        text_format.SetFlowDirection(DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT).unwrap();
    });

    // incremental tab stop
    let text = "Hello,\t\t world!";
    set_text_format(w!("IncrementalTabStop-1.png"), text, |text_format, _| unsafe {
        text_format.SetIncrementalTabStop(1.0).unwrap();
    });
    set_text_format(w!("IncrementalTabStop-10.png"), text, |text_format, _| unsafe {
        text_format.SetIncrementalTabStop(10.0).unwrap();
    });
    set_text_format(w!("IncrementalTabStop-100.png"), text, |text_format, _| unsafe {
        text_format.SetIncrementalTabStop(100.0).unwrap();
    });


    // trimming 添加DWRITE_WORD_WRAPPING_NO_WRAP是因为要超出布局才开始裁切的，换行了就不会超出了
    let text = "ssstart Hello world center Hello world!";
    set_text_format(w!("DWRITE_TRIMMING_GRANULARITY_NONE.png"), text, |text_format, writer_factor| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        let sign = writer_factor.CreateEllipsisTrimmingSign(text_format).unwrap();
        text_format.SetTrimming(&DWRITE_TRIMMING {
            granularity: DWRITE_TRIMMING_GRANULARITY_NONE,
            // delimiter: 'r' as u32,
            delimiter: 0,
            delimiterCount: 1,
        }, &sign).unwrap();
    });
    set_text_format(w!("DWRITE_TRIMMING_GRANULARITY_CHARACTER.png"), text, |text_format, writer_factor| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        let sign = writer_factor.CreateEllipsisTrimmingSign(text_format).unwrap();
        text_format.SetTrimming(&DWRITE_TRIMMING {
            granularity: DWRITE_TRIMMING_GRANULARITY_CHARACTER,
            // delimiter: 'r' as u32,
            // delimiterCount: 2,
            delimiter: 0,
            delimiterCount: 0,
        }, &sign).unwrap();
    });
    set_text_format(w!("DWRITE_TRIMMING_GRANULARITY_WORD.png"), text, |text_format, writer_factor| unsafe {
        text_format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap();
        let sign = writer_factor.CreateEllipsisTrimmingSign(text_format).unwrap();
        text_format.SetTrimming(&DWRITE_TRIMMING {
            granularity: DWRITE_TRIMMING_GRANULARITY_WORD,
            // delimiter: 'r' as u32,
            // delimiterCount: 2,
            delimiter: 0,
            delimiterCount: 0,
        }, &sign).unwrap();
    });

    // line spacing
    let text = "Hello,world!Hello,world!\nHello,world!Hello,world!\nHello,world!Hello,world!\nHello,world!";
    set_text_format(w!("DWRITE_LINE_SPACING_METHOD_DEFAULT.png"), text, |text_format, _| unsafe {
        text_format.SetLineSpacing(DWRITE_LINE_SPACING_METHOD_DEFAULT, 100.0, 100.0).unwrap();
    });
    set_text_format(w!("DWRITE_LINE_SPACING_METHOD_UNIFORM.png"), text, |text_format, _| unsafe {
        text_format.SetLineSpacing(DWRITE_LINE_SPACING_METHOD_UNIFORM, 40.0, 40.0).unwrap();
    });
    set_text_format(w!("DWRITE_LINE_SPACING_METHOD_PROPORTIONAL.png"), text, |text_format, _| unsafe {
        text_format.SetLineSpacing(DWRITE_LINE_SPACING_METHOD_PROPORTIONAL, 2.0, 1.0).unwrap();
    });
}