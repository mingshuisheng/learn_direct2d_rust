use std::iter::once;

use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F};
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_CLIP;
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_FEATURE, DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS, DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS, DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING, DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS, DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES, DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES, DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION, DWRITE_FONT_FEATURE_TAG_LINING_FIGURES, DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS, DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING, DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING, DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK, DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS, DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES, DWRITE_FONT_FEATURE_TAG_ORDINALS, DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS, DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS, DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH, DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES, DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS, DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS, DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES, DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS, DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS, DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS, DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES, DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES, DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1, DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_ULTRA_BLACK, DWRITE_GLYPH_RUN, DWRITE_MEASURING_MODE_NATURAL, DWRITE_TEXT_RANGE, DWRITE_WORD_WRAPPING_NO_WRAP, DWriteCreateFactory, IDWriteFactory2};

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

    let text = "Hello, World!";
    let text_length = text.chars().count() as u32;
    let text_vec = text.encode_utf16().chain(once(0)).collect::<Vec<u16>>();
    let text_format = unsafe {
        write_factory.CreateTextFormat(
            font_family,
            None,
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            font_size,
            w!(""),
        ).unwrap()
    };

    let create_text_layout = || unsafe {
        write_factory.CreateTextLayout(&text_vec, &text_format, 640.0, 480.0).unwrap()
    };

    let origin = D2D_POINT_2F {
        x: 0.0,
        y: 0.0,
    };

    graphic.draw_and_save((640, 480), w!("underline.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();

        let text_layout = create_text_layout();
        text_layout.SetUnderline(true, DWRITE_TEXT_RANGE { startPosition: 0, length: text_length }).unwrap();
        ctx.DrawTextLayout(origin, &text_layout, &brush, D2D1_DRAW_TEXT_OPTIONS_CLIP);
    });

    graphic.draw_and_save((640, 480), w!("strikethrough.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        let text_layout = create_text_layout();
        text_layout.SetStrikethrough(true, DWRITE_TEXT_RANGE { startPosition: 0, length: text_length }).unwrap();
        ctx.DrawTextLayout(origin, &text_layout, &brush, D2D1_DRAW_TEXT_OPTIONS_CLIP);
    });

    graphic.draw_and_save((640, 480), w!("typography.png"), |ctx| unsafe {
        ctx.Clear(Some(&graphic_color));
        //text
        let brush = ctx.CreateSolidColorBrush(&text_color, None).unwrap();
        // let text_layout = create_text_layout();
        // ctx.DrawTextLayout(origin, &text_layout, &brush, D2D1_DRAW_TEXT_OPTIONS_CLIP);
        let glyph_run = DWRITE_GLYPH_RUN{
            fontFace: Default::default(),
            fontEmSize: 40.0,
            glyphCount: text_length * 2,
            // glyphIndices: text_vec.as_ptr(),
            // glyphAdvances: std::ptr::null(),
            // glyphOffsets: std::ptr::null(),
            // isSideways: Default::default(),
            // bidiLevel: 0,
            ..Default::default()
        };
        ctx.DrawGlyphRun2(origin, &glyph_run, None, &brush, DWRITE_MEASURING_MODE_NATURAL);
    });
}