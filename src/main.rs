use windows::core::{IntoParam, w};
use windows::Win32::Graphics::Direct2D::{D2D1_DASH_STYLE_SOLID, D2D1_LINE_JOIN, D2D1_LINE_JOIN_MITER, D2D1_LINE_JOIN_MITER_OR_BEVEL, D2D1_STROKE_STYLE_PROPERTIES, ID2D1Brush, ID2D1DeviceContext, ID2D1Factory1, ID2D1StrokeStyle};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let style1 = create_stroke_style(&graphic.d2d_factory, D2D1_LINE_JOIN_MITER, 0.0);
    let style4 = create_stroke_style(&graphic.d2d_factory, D2D1_LINE_JOIN_MITER_OR_BEVEL, 0.0);

    let miter_limit = 1.0;
    let style5 = create_stroke_style(&graphic.d2d_factory, D2D1_LINE_JOIN_MITER, miter_limit);
    let style8 = create_stroke_style(&graphic.d2d_factory, D2D1_LINE_JOIN_MITER_OR_BEVEL, miter_limit);

    graphic.draw_and_save((640, 480), w!("output.png"), |ctx| unsafe {
        ctx.Clear(Some(&D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }));

        let brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }, None).unwrap();

        let rect = D2D_RECT_F {
            left: 50.0,
            top: 50.0,
            right: 150.0,
            bottom: 150.0,
        };

        let stroke_width = 20.0;
        let x_offset = 150.0;
        let y_offset = 150.0;
        draw_rectangle(ctx, &rect, &brush, stroke_width, &style1, x_offset * 0.0, 0.0);
        draw_rectangle(ctx, &rect, &brush, stroke_width, &style4, x_offset * 1.0, 0.0);

        draw_rectangle(ctx, &rect, &brush, stroke_width, &style5, x_offset * 0.0, y_offset);
        draw_rectangle(ctx, &rect, &brush, stroke_width, &style8, x_offset * 1.0, y_offset);
    });
}

fn draw_rectangle(ctx: &ID2D1DeviceContext, rect: &D2D_RECT_F, brush: impl IntoParam<ID2D1Brush>, stroke_width: f32, style: &ID2D1StrokeStyle, x_offset: f32, y_offset: f32) {
    let mut rect = rect.clone();
    rect.left += x_offset;
    rect.right += x_offset;
    rect.top += y_offset;
    rect.bottom += y_offset;
    unsafe {
        ctx.DrawRectangle(&rect, brush, stroke_width, style);
    }
}

fn create_stroke_style(d2d_factory: &ID2D1Factory1, line_join: D2D1_LINE_JOIN, miter_limit: f32) -> ID2D1StrokeStyle {
    unsafe {
        d2d_factory.CreateStrokeStyle(&D2D1_STROKE_STYLE_PROPERTIES {
            // startCap: D2D1_CAP_STYLE_ROUND,
            // endCap: D2D1_CAP_STYLE_TRIANGLE,
            // dashCap: D2D1_CAP_STYLE_TRIANGLE,
            lineJoin: line_join,
            miterLimit: miter_limit,
            dashStyle: D2D1_DASH_STYLE_SOLID,
            // dashOffset: 1.0,
            ..Default::default()
        }, None).unwrap()
    }
}